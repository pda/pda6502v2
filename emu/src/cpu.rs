use std::fmt;

use crate::bus;
use crate::isa;

// A 65C02-like CPU
pub struct Cpu {
    bus: bus::Bus,
    pc: u16, // program counter
    sp: u8,  // stack pointer
    a: u8,   // accumulator
    x: u8,   // X register
    y: u8,   // Y register
    sr: u8,  // status register

    optab: [Option<isa::Opcode>; 256],
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::asm::{val, Assembler, Operand};
    use crate::isa::{AddressMode, Mnemonic, OpcodeByMnemonicAndAddressMode};

    #[test]
    fn test_nop() {
        let mut cpu = Cpu::new(bus::Bus::default());
        cpu.execute(op(Mnemonic::Nop, AddressMode::Implied));
        assert_eq!(stat(&cpu.sr), "nv-bdizc");
    }

    #[test]
    fn test_inx() {
        let mut cpu = Cpu::new(bus::Bus::default());
        cpu.x = 0xFF;
        let op = op(Mnemonic::Inx, AddressMode::Implied);
        cpu.execute(op);
        assert_eq!(cpu.x, 0x00);
        assert_eq!(stat(&cpu.sr), "nv-bdiZc");
        cpu.execute(op);
        assert_eq!(cpu.x, 0x01);
        assert_eq!(stat(&cpu.sr), "nv-bdizc");
    }

    #[test]
    fn test_ldx() {
        let mut cpu = Cpu::new(bus::Bus::default());
        cpu.y = 0x02; // for testing AddressMode::ZeropageY & AddressMode::AbsoluteY
        cpu.bus.write(0x01FF, 0x11); // for testing AddressMode::Absolute
        cpu.bus.write(0x0201, 0x22); // for testing AddressMode::AbsoluteY

        let mut asm = Assembler::new();
        cpu.bus.load(
            cpu.pc,
            asm.ldx(Operand::Imm(0xAA))
                .ldx(Operand::Imm(0x00))
                .ldx(Operand::Z(0x04))
                .ldx(Operand::ZY(0x04))
                .ldx(Operand::Abs(val(0x01FF)))
                .ldx(Operand::AbsY(val(0x01FF)))
                .print_listing()
                .assemble()
                .unwrap(),
        );

        cpu.step(); // LDX #$AA
        println!("{:?}", cpu);
        assert_eq!(cpu.x, 0xAA);
        assert_eq!(stat(&cpu.sr), "Nv-bdizc");

        cpu.step(); // LDX #$00
        println!("{:?}", cpu);
        assert_eq!(cpu.x, 0x00);
        assert_eq!(stat(&cpu.sr), "nv-bdiZc");

        cpu.step(); // LDX $04
        println!("{:?}", cpu);
        assert_eq!(cpu.x, 0xA6);
        assert_eq!(stat(&cpu.sr), "Nv-bdizc");

        cpu.step(); // LDX $04,Y
        println!("{:?}", cpu);
        assert_eq!(cpu.x, 0xB6, "{:#04X} != {:#04X}", cpu.x, 0xB6);
        assert_eq!(stat(&cpu.sr), "Nv-bdizc");

        cpu.step(); // LDX $01FF ; Y=2
        println!("{:?}", cpu);
        assert_eq!(cpu.x, 0x11, "{:#04X} != {:#04X}", cpu.x, 0x11);
        assert_eq!(stat(&cpu.sr), "nv-bdizc");

        cpu.step(); // LDX $01FF,Y ; Y=2
        println!("{:?}", cpu);
        assert_eq!(cpu.x, 0x22, "{:#04X} != {:#04X}", cpu.x, 0x22);
        assert_eq!(stat(&cpu.sr), "nv-bdizc");
    }

    fn op(m: Mnemonic, am: AddressMode) -> isa::Opcode {
        OpcodeByMnemonicAndAddressMode::build().get(m, am).unwrap()
    }
}

impl Cpu {
    pub fn new(bus: bus::Bus) -> Cpu {
        Cpu {
            bus,
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            sr: 0,
            optab: build_opcode_table(),
        }
    }

    // Reset internal CPU state, as if the reset line had been asserted.
    pub fn reset(&mut self) {
        self.pc = self.read_u16(0xFFFC);
        self.sp = 0x00;
        self.a = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        self.sr = 0x00; // TODO: manual says xx1101xx, so set 00110100?
    }

    // Load and execute a single instruction.
    pub fn step(&mut self) {
        match self.optab[self.read_pc_u8() as usize] {
            None => panic!("illegal opcode"),
            Some(opcode) => self.execute(opcode),
        }
    }

    // Execute an instruction, reading the operands for its address mode from the bus.
    fn execute(&mut self, opcode: isa::Opcode) {
        use isa::AddressMode::*;
        use isa::Mnemonic as M;
        use isa::OpValue;

        println!("{:?}", opcode);
        match opcode.mnemonic {
            // M::Adc => {}
            // M::And => {}
            // M::Asl => {}
            // M::Bcc => {}
            // M::Bcs => {}
            // M::Beq => {}
            // M::Bit => {}
            // M::Bmi => {}
            // M::Bne => {}
            // M::Bpl => {}
            // M::Brk => {}
            // M::Bvc => {}
            // M::Bvs => {}
            // M::Clc => {}
            // M::Cld => {}
            // M::Cli => {}
            // M::Clv => {}
            // M::Cmp => {}
            // M::Cpx => {}
            // M::Cpy => {}
            // M::Dec => {}
            // M::Dex => {}
            // M::Dey => {}
            // M::Eor => {}
            // M::Inc => {}
            M::Inx => {
                match opcode.mode {
                    Implied => self.x = self.x.wrapping_add(1),
                    _ => panic!("illegal AddressMode: {:?}", opcode),
                }
                self.update_status(self.x);
            }
            // M::Iny => {}
            M::Jmp => match self.read_operand(opcode.mode) {
                OpValue::U16(addr) => self.pc = addr,
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            // M::Jsr => {}
            // M::Lda => {}
            M::Ldx => {
                match self.read_operand(opcode.mode) {
                    OpValue::None => panic!("illegal AddressMode: {:?}", opcode),
                    OpValue::U8(val) => self.x = val,
                    OpValue::U16(addr) => self.x = self.bus.read(addr),
                }
                self.update_status(self.x);
            }
            // M::Ldy => {}
            // M::Lsr => {}
            M::Nop => {}
            // M::Ora => {}
            // M::Pha => {}
            // M::Php => {}
            // M::Pla => {}
            // M::Plp => {}
            // M::Rol => {}
            // M::Ror => {}
            // M::Rti => {}
            // M::Rts => {}
            // M::Sbc => {}
            // M::Sec => {}
            // M::Sed => {}
            // M::Sei => {}
            // M::Sta => {}
            // M::Stx => {}
            // M::Sty => {}
            // M::Tax => {}
            // M::Tay => {}
            // M::Tsx => {}
            // M::Txa => {}
            // M::Txs => {}
            // M::Tya => {}
            other => todo!("{:?}", other),
        }
    }

    /// Read a u16 in little-endian order from the bus
    fn read_u16(&self, addr: u16) -> u16 {
        let lo = self.bus.read(addr as u16);
        let hi = self.bus.read(addr.wrapping_add(1) as u16);
        ((hi as u16) << 8) | (lo as u16)
    }

    /// Read u8 from address pointed to by PC, incrementing PC
    fn read_pc_u8(&mut self) -> u8 {
        let val = self.bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        val
    }

    /// Read u16 from address pointed to by PC, incrementing PC
    fn read_pc_u16(&mut self) -> u16 {
        let val = self.read_u16(self.pc);
        self.pc = self.pc.wrapping_add(2);
        val
    }

    /// Read an index-offset zero-page address pointed to by PC, returning as u16 address after
    /// incrementing PC by one
    fn read_pc_zp(&mut self, index: u8) -> u16 {
        self.read_pc_u8().wrapping_add(index) as u16
    }

    /// Reads the operand from self.bus, following indirection/indexing where necessary, returning
    /// an address or immediate value, and incrementing PC.
    fn read_operand(&mut self, mode: isa::AddressMode) -> isa::OpValue {
        use isa::AddressMode::*;
        use isa::OpValue as OV;

        let addr = match mode {
            Absolute => OV::U16(self.read_pc_u16()),
            AbsoluteX => OV::U16(self.read_pc_u16().wrapping_add(self.x as u16)),
            AbsoluteY => OV::U16(self.read_pc_u16().wrapping_add(self.y as u16)),
            Accumulator => OV::None,
            Immediate => OV::U8(self.read_pc_u8()),
            Implied => OV::None,
            Indirect => {
                let ptr = self.read_pc_u16();
                OV::U16(self.read_u16(ptr))
            }
            IndirectY => {
                let ptr = self.read_pc_zp(0);
                OV::U16(self.read_u16(ptr).wrapping_add(self.y as u16))
            }
            Relative => {
                // #![feature(mixed_integer_ops)]
                // OV::U16(self.pc.wrapping_add_signed(self.read_pc_u8().into()))
                let base = self.pc as i16;
                let offset: i16 = (self.read_pc_u8() as i8).into(); // not sure if this is legit
                OV::U16((base + offset) as u16)
            }
            XIndirect => {
                let ptr = self.read_pc_zp(self.x);
                OV::U16(self.read_u16(ptr))
            }
            Zeropage => OV::U16(self.read_pc_zp(0)),
            ZeropageX => OV::U16(self.read_pc_zp(self.x)),
            ZeropageY => OV::U16(self.read_pc_zp(self.y)),
        };

        addr
    }

    fn update_status(&mut self, val: u8) {
        let z_bit = 1;
        if val == 0 {
            self.sr |= 1 << z_bit;
        } else {
            self.sr &= !(1 << z_bit);
        }

        let n_bit = 7;
        if (val as i8) < 0 {
            self.sr |= 1 << n_bit;
        } else {
            self.sr &= !(1 << n_bit);
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let stat = stat(&self.sr);
        f.write_fmt(format_args!(
            "Cpu {{ SR: {} PC: ${:04X} SP: ${:02X} A: ${:02X} X: ${:02X} Y: ${:02X} }}",
            stat, self.pc, self.sp, self.a, self.x, self.y,
        ))
    }
}

// a string representation of the 8-bit status register;
// enabled bits are upper case, disabled bits are lower case.
fn stat(sr: &u8) -> String {
    "nv-bdizc"
        .chars()
        .enumerate()
        .map(|(i, x)| {
            if sr >> (7 - i) & 1 == 1 {
                x.to_ascii_uppercase()
            } else {
                x
            }
        })
        .collect()
}

// Build an array of isa::Opcode indexed by by their u8 opcode.
pub fn build_opcode_table() -> [Option<isa::Opcode>; 256] {
    let mut optab = [None; 256];
    for opcode in isa::opcode_list() {
        optab[opcode.code as usize] = Some(opcode);
    }
    optab
}

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

    #[test]
    fn test_adc() {
        let mut cpu = Cpu::new(bus::Bus::default());
        cpu.a = 0x10; // starting value
        cpu.x = 0x21; // for testing X-indexed address modes
        cpu.y = 0x46; // for testing Y-indexed address modes
        cpu.bus.write(0x00F0, 0xEF); // for testing zero-page address mode
        cpu.bus.write(0x00F1, 0x70); // for testing zp,X address mode
        cpu.bus.write(0x00F2, 0x37); // for testing X,ind address mode (ptr LO)
        cpu.bus.write(0x00F3, 0x12); // for testing X,ind address mode (ptr HI)
        cpu.bus.write(0x00F4, 0xF2); // for testing ind,Y address mode (ptr LO)
        cpu.bus.write(0x00F5, 0x11); // for testing ind,Y address mode (ptr HI)
        cpu.bus.write(0x1234, 0x84); // for testing absolute address mode
        cpu.bus.write(0x1235, 0xFA); // for testing abs,X address mode
        cpu.bus.write(0x1236, 0x00); // for testing abs,X address mode
        cpu.bus.write(0x1237, 0x42); // for testing X,ind address mode (val)
        cpu.bus.write(0x1238, 0xBC); // for testing ind,Y address mode (val)

        use crate::asm::Operand::{Abs, AbsX, AbsY, Imm, IndY, XInd, Z, ZX};
        let mut asm = Assembler::new();
        cpu.bus.load(
            cpu.pc,
            asm.adc(Imm(0x11)) //          C:0+A:$10+#$11                            =$21+C:0
                .adc(Z(0xF0)) //           C:0+A:$21+[$F0→#$EF]                      =$10+C:1
                .adc(ZX(0xD0)) //          C:1+A:$10+[$D0+X:$21→$F1→#$70]            =$81+C:0(V)
                .adc(Abs(val(0x1234))) //  C:0+A:$81+[$1234→#$84]                    =$05+C:1(V)
                .adc(AbsX(val(0x1214))) // C:1+A:$05+[$1214+X:$21→$1235→#$FA]        =$00+C:1
                .adc(AbsY(val(0x12F0))) // C:1+A:$00+[$12F0+Y:$46→$1236→#$00]        =$01+C:0
                .adc(XInd(0xD1)) //        C:0+A:$01+[($D1+X:$21)→($F2)→$1237→#$42]  =$43+C:0
                .adc(IndY(0xF4)) //        C:0+A:$43+[($F4)+Y→$11F2+Y:$46→$1238→#$BC]=$FF+C:0
                .print_listing()
                .assemble()
                .unwrap(),
        );

        cpu.step(); // ADC #$11
        println!("{:?}", cpu);
        assert_eq!(cpu.a, 0x21, "{:#04X} != {:#04X}", cpu.a, 0x21);
        assert_eq!(stat(&cpu.sr), "nv-bdizc");

        cpu.step(); // ADC $F0
        println!("{:?}", cpu);
        assert_eq!(cpu.a, 0x10, "{:#04X} != {:#04X}", cpu.a, 0x10);
        assert_eq!(stat(&cpu.sr), "nv-bdizC");

        cpu.step(); // ADC $D0,X
        println!("{:?}", cpu);
        assert_eq!(cpu.a, 0x81, "{:#04X} != {:#04X}", cpu.a, 0x81);
        assert_eq!(stat(&cpu.sr), "NV-bdizc");

        cpu.step(); // ADC $1234
        println!("{:?}", cpu);
        assert_eq!(cpu.a, 0x05, "{:#04X} != {:#04X}", cpu.a, 0x05);
        assert_eq!(stat(&cpu.sr), "nV-bdizC");

        cpu.step(); // ADC $1214,X
        println!("{:?}", cpu);
        assert_eq!(cpu.a, 0x00, "{:#04X} != {:#04X}", cpu.a, 0x00);
        assert_eq!(stat(&cpu.sr), "nv-bdiZC");

        cpu.step(); // ADC $12F0,Y
        println!("{:?}", cpu);
        assert_eq!(cpu.a, 0x01, "{:#04X} != {:#04X}", cpu.a, 0x01);
        assert_eq!(stat(&cpu.sr), "nv-bdizc");

        cpu.step(); // ADC ($D1,X)
        println!("{:?}", cpu);
        assert_eq!(cpu.a, 0x43, "{:#04X} != {:#04X}", cpu.a, 0x43);
        assert_eq!(stat(&cpu.sr), "nv-bdizc");

        cpu.step(); // ADC ($F4),Y
        println!("{:?}", cpu);
        assert_eq!(cpu.a, 0xFF, "{:#04X} != {:#04X}", cpu.a, 0xFF);
        assert_eq!(stat(&cpu.sr), "Nv-bdizc");
    }

    #[test]
    fn test_and() {
        let mut cpu = Cpu::new(bus::Bus::default());
        cpu.a = 0b10011001; // starting value

        use crate::asm::Operand::Imm;
        let mut asm = Assembler::new();
        cpu.bus.load(
            cpu.pc,
            asm.and(Imm(0b11110000)).print_listing().assemble().unwrap(),
        );

        cpu.step(); // ADC #$A0
        println!("{:?}", cpu);
        assert_eq!(cpu.a, 0b10010000, "{:#04X} != {:#04X}", cpu.a, 0b10010000);
        assert_eq!(stat(&cpu.sr), "Nv-bdizc");
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
        self.sr = 0b00110100; // W65C02S manual §3.1 Reset says xx1101xx
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
            M::Adc => {
                let a = self.a;
                let b = self.read_operand_value(opcode);
                let sum16 = (self.carry() as u16) + (a as u16) + (b as u16);
                let sum = sum16 as u8;
                self.a = sum;
                self.set_sr_bit(StatusMask::Carry, sum < a);
                self.set_sr_bit(StatusMask::Overflow, ((a ^ sum) & (b ^ sum)) >> 7 != 0);
                self.update_sr_z_n(self.a);
            }
            M::And => {
                self.a &= self.read_operand_value(opcode);
                self.update_sr_z_n(self.a);
            }
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
                self.update_sr_z_n(self.x);
            }
            // M::Iny => {}
            M::Jmp => match self.read_operand(opcode.mode) {
                OpValue::U16(addr) => self.pc = addr,
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            // M::Jsr => {}
            // M::Lda => {}
            M::Ldx => {
                self.x = self.read_operand_value(opcode);
                self.update_sr_z_n(self.x);
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

    fn read_operand_value(&mut self, opcode: isa::Opcode) -> u8 {
        use isa::OpValue;
        match self.read_operand(opcode.mode) {
            OpValue::U8(val) => val,
            OpValue::U16(addr) => self.bus.read(addr),
            OpValue::None => panic!("illegal AddressMode: {:?}", opcode),
        }
    }

    /// Update the Status Register's Zero and Negative bits based on the specified value.
    fn update_sr_z_n(&mut self, val: u8) {
        self.set_sr_bit(StatusMask::Zero, val == 0);
        self.set_sr_bit(StatusMask::Negative, (val as i8) < 0);
    }

    fn set_sr_bit(&mut self, mask: StatusMask, val: bool) {
        let m = mask as u8;
        if val {
            self.sr |= m
        } else {
            self.sr &= !m
        }
    }

    fn carry(&mut self) -> u8 {
        let bit = StatusBit::Carry as u8;
        let mask = StatusMask::Carry as u8;
        (self.sr & mask) >> bit
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

#[repr(u8)]
enum StatusBit {
    Carry = 0,
    Zero = 1,
    Interrupt = 2,
    Decimal = 3,
    Break = 4,
    // unused bit 5
    Overflow = 6,
    Negative = 7,
}

#[allow(unused)]
#[repr(u8)]
enum StatusMask {
    Carry = 1 << StatusBit::Carry as u8,
    Zero = 1 << StatusBit::Zero as u8,
    Interrupt = 1 << StatusBit::Interrupt as u8,
    Decimal = 1 << StatusBit::Decimal as u8,
    Break = 1 << StatusBit::Break as u8,
    Overflow = 1 << StatusBit::Overflow as u8,
    Negative = 1 << StatusBit::Negative as u8,
}

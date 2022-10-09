use std::fmt;

use crate::bus;
use crate::isa;

// A 65C02-like CPU
pub struct Cpu {
    pub bus: bus::Bus,
    pub pc: u16, // program counter
    pub sp: u8,  // stack pointer
    pub a: u8,   // accumulator
    pub x: u8,   // X register
    pub y: u8,   // Y register
    pub sr: u8,  // status register

    optab: [Option<isa::Opcode>; 256],
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
            M::Asl => {
                let result: u8;
                let carry: u8;
                match self.read_operand(opcode.mode) {
                    OpValue::None => {
                        result = self.a << 1;
                        carry = self.a >> 7;
                        self.a = result;
                    }
                    OpValue::U16(addr) => {
                        let x = self.bus.read(addr);
                        result = x << 1;
                        carry = x >> 7;
                        self.bus.write(addr, result);
                    }
                    _ => panic!("illegal AddressMode: {:?}", opcode),
                }
                self.update_sr_z_n(result);
                self.set_sr_bit(StatusMask::Carry, carry == 1);
            }
            M::Bcc => {
                if !self.get_sr_bit(StatusMask::Carry) {
                    match self.read_operand(opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Bcs => {
                if self.get_sr_bit(StatusMask::Carry) {
                    match self.read_operand(opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Beq => {
                if self.get_sr_bit(StatusMask::Zero) {
                    match self.read_operand(opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Bit => {
                let operand = self.read_operand_value(opcode);
                let r = self.a & operand;
                self.set_sr_bit(StatusMask::Negative, r & StatusMask::Negative as u8 != 0);
                self.set_sr_bit(StatusMask::Overflow, r & StatusMask::Overflow as u8 != 0);
                self.set_sr_bit(StatusMask::Zero, r == 0);
            }
            M::Bmi => {
                if self.get_sr_bit(StatusMask::Negative) {
                    match self.read_operand(opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Bne => {
                if !self.get_sr_bit(StatusMask::Zero) {
                    match self.read_operand(opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Bpl => {
                if !self.get_sr_bit(StatusMask::Negative) {
                    match self.read_operand(opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            // M::Brk => {}
            M::Bvc => {
                if !self.get_sr_bit(StatusMask::Overflow) {
                    match self.read_operand(opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Bvs => {
                if self.get_sr_bit(StatusMask::Overflow) {
                    match self.read_operand(opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
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
                let offset: i16 = (self.read_pc_u8() as i8).into();
                let base = self.pc as i16;
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

    pub fn set_sr_bit(&mut self, mask: StatusMask, val: bool) {
        let m = mask as u8;
        if val {
            self.sr |= m
        } else {
            self.sr &= !m
        }
    }

    pub fn get_sr_bit(&mut self, mask: StatusMask) -> bool {
        self.sr & mask as u8 != 0
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
pub fn stat(sr: &u8) -> String {
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

// TODO: maybe StatusBit and StatusMask belong in isa.rs
#[repr(u8)]
pub enum StatusBit {
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
pub enum StatusMask {
    Carry = 1 << StatusBit::Carry as u8,
    Zero = 1 << StatusBit::Zero as u8,
    Interrupt = 1 << StatusBit::Interrupt as u8,
    Decimal = 1 << StatusBit::Decimal as u8,
    Break = 1 << StatusBit::Break as u8,
    Overflow = 1 << StatusBit::Overflow as u8,
    Negative = 1 << StatusBit::Negative as u8,
}

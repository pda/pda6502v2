use std::fmt;

use crate::bus;
use crate::dec;
use crate::isa;

// A 65C02-like CPU
pub struct Cpu {
    pub pc: u16, // program counter
    pub s: u8,   // stack pointer
    pub a: u8,   // accumulator
    pub x: u8,   // X register
    pub y: u8,   // Y register
    pub p: u8,   // processor status

    decoder: dec::Decoder,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            s: 0,
            a: 0,
            x: 0,
            y: 0,
            p: 0,
            decoder: dec::Decoder::new(),
        }
    }

    // Reset internal CPU state, as if the reset line had been asserted.
    pub fn reset(&mut self, bus: &bus::Bus) {
        self.pc = self.read_u16(bus, VEC_RES);
        self.s = 0x00;
        self.a = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        self.p = 0b00110100; // W65C02S manual §3.1 Reset says xx1101xx
    }

    pub fn fetch(&self, bus: &bus::Bus) -> Option<isa::Opcode> {
        self.decoder.opcode(bus.read(self.pc))
    }

    // Load and execute a single instruction.
    pub fn step(&mut self, bus: &mut bus::Bus) {
        match self.fetch(bus) {
            None => panic!("illegal opcode"),
            Some(opcode) => self.execute(opcode, bus),
        }
    }

    // Execute an instruction, reading the operands for its address mode from the bus.
    fn execute(&mut self, opcode: isa::Opcode, bus: &mut bus::Bus) {
        use isa::AddressMode::*;
        use isa::Mnemonic as M;
        use isa::OpValue;

        // progress PC for the fetched opcode
        self.pc = self.pc.wrapping_add(1);

        match opcode.mnemonic {
            M::Adc => {
                let a = self.a;
                let b = self.read_operand_value(bus, opcode);
                let sum16 = (self.carry() as u16) + (a as u16) + (b as u16);
                let sum = sum16 as u8;
                self.a = sum;
                self.update_p_z_n(sum);
                self.set_p_bit(StatusMask::Carry, sum < a);

                // Whether the sign of `a` and `sum` differs AND the sign of `b` and `sum` differs.
                // This implies that `a` and `b` are same-sign, but `sum` is other-sign: overflow.
                //
                // a b s | a^s b^s | & | !=0 | Overflow?
                // -------------------------------------
                // 0 0 0 |  0   0  | 0 |  0  | no
                // 0 0 1 |  1   1  | 1 |  1  | yes
                // 0 1 0 |  0   1  | 0 |  0  | no
                // 0 1 1 |  1   0  | 0 |  0  | no
                // 1 0 0 |  1   0  | 0 |  0  | no
                // 1 0 1 |  0   1  | 0 |  0  | no
                // 1 1 0 |  1   1  | 1 |  1  | yes
                // 1 1 1 |  0   0  | 0 |  0  | no
                self.set_p_bit(StatusMask::Overflow, ((a ^ sum) & (b ^ sum)) >> 7 != 0);
            }
            M::And => {
                self.a &= self.read_operand_value(bus, opcode);
                self.update_p_z_n(self.a);
            }
            M::Asl => {
                let result: u8;
                let carry: u8;
                match self.read_operand(bus, opcode.mode) {
                    OpValue::None => {
                        result = self.a << 1;
                        carry = self.a >> 7;
                        self.a = result;
                    }
                    OpValue::U16(addr) => {
                        let x = bus.read(addr);
                        result = x << 1;
                        carry = x >> 7;
                        bus.write(addr, result);
                    }
                    _ => panic!("illegal AddressMode: {:?}", opcode),
                }
                self.update_p_z_n(result);
                self.set_p_bit(StatusMask::Carry, carry == 1);
            }
            M::Bcc => {
                if !self.get_p_bit(StatusMask::Carry) {
                    match self.read_operand(bus, opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Bcs => {
                if self.get_p_bit(StatusMask::Carry) {
                    match self.read_operand(bus, opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Beq => {
                if self.get_p_bit(StatusMask::Zero) {
                    match self.read_operand(bus, opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Bit => {
                let operand = self.read_operand_value(bus, opcode);
                let r = self.a & operand;
                self.set_p_bit(StatusMask::Negative, r & StatusMask::Negative as u8 != 0);
                self.set_p_bit(StatusMask::Overflow, r & StatusMask::Overflow as u8 != 0);
                self.set_p_bit(StatusMask::Zero, r == 0);
            }
            M::Bmi => {
                if self.get_p_bit(StatusMask::Negative) {
                    match self.read_operand(bus, opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Bne => {
                if !self.get_p_bit(StatusMask::Zero) {
                    match self.read_operand(bus, opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Bpl => {
                if !self.get_p_bit(StatusMask::Negative) {
                    match self.read_operand(bus, opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Brk => match opcode.mode {
                Implied => {
                    self.push_addr(bus, self.pc + 1);
                    self.push(bus, self.p | StatusMask::Break as u8);
                    self.p |= StatusMask::Interrupt as u8;
                    self.pc = self.read_u16(bus, VEC_IRQ);
                }
                _ => panic!("illegal AddressMode: {opcode:?}"),
            },
            M::Bvc => {
                if !self.get_p_bit(StatusMask::Overflow) {
                    match self.read_operand(bus, opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Bvs => {
                if self.get_p_bit(StatusMask::Overflow) {
                    match self.read_operand(bus, opcode.mode) {
                        OpValue::U16(addr) => self.pc = addr,
                        _ => panic!("illegal AddressMode: {:?}", opcode),
                    }
                } else {
                    self.pc += 1; // skip operand
                }
            }
            M::Clc => match opcode.mode {
                Implied => self.set_p_bit(StatusMask::Carry, false),
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            M::Cld => match opcode.mode {
                Implied => self.set_p_bit(StatusMask::Decimal, false),
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            M::Cli => match opcode.mode {
                Implied => self.set_p_bit(StatusMask::Interrupt, false),
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            M::Clv => match opcode.mode {
                Implied => self.set_p_bit(StatusMask::Overflow, false),
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            M::Cmp => {
                let result = self.a.wrapping_sub(self.read_operand_value(bus, opcode));
                self.update_p_z_n(result);
                self.set_p_bit(StatusMask::Carry, result > self.a);
            }
            M::Cpx => {
                let result = self.x.wrapping_sub(self.read_operand_value(bus, opcode));
                self.update_p_z_n(result);
                self.set_p_bit(StatusMask::Carry, result > self.x);
            }
            M::Cpy => {
                let result = self.y.wrapping_sub(self.read_operand_value(bus, opcode));
                self.update_p_z_n(result);
                self.set_p_bit(StatusMask::Carry, result > self.y);
            }
            M::Dec => match self.read_operand(bus, opcode.mode) {
                OpValue::U16(addr) => {
                    let result = bus.read(addr).wrapping_sub(1);
                    bus.write(addr, result);
                    self.update_p_z_n(result);
                }
                _ => panic!("illegal AddressMode: {opcode:?}"),
            },
            M::Dex => match opcode.mode {
                Implied => {
                    self.x = self.x.wrapping_sub(1);
                    self.update_p_z_n(self.x);
                }
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            M::Dey => match opcode.mode {
                Implied => {
                    self.y = self.y.wrapping_sub(1);
                    self.update_p_z_n(self.y);
                }
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            M::Eor => {
                self.a ^= self.read_operand_value(bus, opcode);
                self.update_p_z_n(self.a);
            }
            M::Inc => match self.read_operand(bus, opcode.mode) {
                OpValue::U16(addr) => {
                    let result = bus.read(addr).wrapping_add(1);
                    bus.write(addr, result);
                    self.update_p_z_n(result);
                }
                _ => panic!("illegal AddressMode: {opcode:?}"),
            },
            M::Inx => match opcode.mode {
                Implied => {
                    self.x = self.x.wrapping_add(1);
                    self.update_p_z_n(self.x);
                }
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            M::Iny => match opcode.mode {
                Implied => {
                    self.y = self.y.wrapping_add(1);
                    self.update_p_z_n(self.y);
                }
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            M::Jmp => match self.read_operand(bus, opcode.mode) {
                OpValue::U16(addr) => self.pc = addr,
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            M::Jsr => match self.read_operand(bus, opcode.mode) {
                OpValue::U16(addr) => {
                    self.push_addr(bus, self.pc);
                    self.pc = addr;
                }
                _ => panic!("illegal AddressMode: {opcode:?}"),
            },
            M::Lda => {
                self.a = self.read_operand_value(bus, opcode);
                self.update_p_z_n(self.a);
            }
            M::Ldx => {
                self.x = self.read_operand_value(bus, opcode);
                self.update_p_z_n(self.x);
            }
            M::Ldy => {
                self.y = self.read_operand_value(bus, opcode);
                self.update_p_z_n(self.y);
            }
            M::Lsr => match opcode.mode {
                Accumulator => {
                    let before = self.a;
                    let after = before >> 1;
                    self.a = after;
                    self.update_p_z_n(after);
                    self.set_p_bit(StatusMask::Carry, before & 1 == 1);
                }
                _ => match self.read_operand(bus, opcode.mode) {
                    OpValue::U16(addr) => {
                        let before = bus.read(addr);
                        let after = before >> 1;
                        bus.write(addr, after);
                        self.update_p_z_n(after);
                        self.set_p_bit(StatusMask::Carry, before & 1 == 1);
                    }
                    _ => panic!("illegal AddressMode: {opcode:?}"),
                },
            },
            M::Nop => {}
            M::Ora => {
                self.a |= self.read_operand_value(bus, opcode);
                self.update_p_z_n(self.a);
            }
            M::Pha => self.push(bus, self.a),
            M::Php => self.push(bus, self.p | 0b00110000),
            M::Pla => {
                self.a = self.pop(bus);
                self.update_p_z_n(self.a);
            }
            M::Plp => self.p = self.pop(bus) & !0b00110000,
            M::Rol => match opcode.mode {
                Accumulator => {
                    let before = self.a;
                    let after = before << 1 | self.get_p_bit(StatusMask::Carry) as u8;
                    self.a = after;
                    self.update_p_z_n(after);
                    self.set_p_bit(StatusMask::Carry, before & 0b10000000 != 0);
                }
                _ => match self.read_operand(bus, opcode.mode) {
                    OpValue::U16(addr) => {
                        let before = bus.read(addr);
                        let after = before << 1 | self.get_p_bit(StatusMask::Carry) as u8;
                        bus.write(addr, after);
                        self.update_p_z_n(after);
                        self.set_p_bit(StatusMask::Carry, before & 0b10000000 != 0);
                    }
                    _ => panic!("illegal AddressMode: {opcode:?}"),
                },
            },
            M::Ror => match opcode.mode {
                Accumulator => {
                    let before = self.a;
                    let after = before >> 1 | (self.get_p_bit(StatusMask::Carry) as u8) << 7;
                    self.a = after;
                    self.update_p_z_n(after);
                    self.set_p_bit(StatusMask::Carry, before & 0b00000001 != 0);
                }
                _ => match self.read_operand(bus, opcode.mode) {
                    OpValue::U16(addr) => {
                        let before = bus.read(addr);
                        let after = before >> 1 | (self.get_p_bit(StatusMask::Carry) as u8) << 7;
                        bus.write(addr, after);
                        self.update_p_z_n(after);
                        self.set_p_bit(StatusMask::Carry, before & 0b00000001 != 0);
                    }
                    _ => panic!("illegal AddressMode: {opcode:?}"),
                },
            },
            M::Rti => match opcode.mode {
                Implied => {
                    self.p = self.pop(bus) & !(StatusMask::Break as u8) | 1 << 5;
                    self.pc = self.pop_addr(bus);
                }
                _ => panic!("illegal AddressMode: {opcode:?}"),
            },
            M::Rts => match opcode.mode {
                Implied => self.pc = self.pop_addr(bus),
                _ => panic!("illegal AddressMode: {opcode:?}"),
            },
            M::Sbc => {
                let a = self.a;
                let b = self.read_operand_value(bus, opcode);
                let sum16 = (a as i16) - (b as i16) - (!self.get_p_bit(StatusMask::Carry) as i16);
                let sum = sum16 as u8;
                self.a = sum;
                self.update_p_z_n(sum);
                self.set_p_bit(StatusMask::Carry, sum < a);

                // Whether the sign of `a` and `sum` differs AND the sign of `-b` and `sum` differs.
                // This implies that `a` and `-b` are same-sign, but `sum` is other-sign: overflow.
                // Note `(a - b) == (a + -b)` hence using `-b` (or bitwise !b).
                //
                // Truth table for sign bit:
                // a b !b s | a^s !b^s | & | !=0 | Overflow?
                // ----------------------------------------
                // 0 0  1 0 |  0    1  | 0 |  0  | no
                // 0 0  1 1 |  1    0  | 0 |  0  | no
                // 0 1  0 0 |  0    0  | 0 |  0  | no
                // 0 1  0 1 |  1    1  | 1 |  1  | yes
                // 1 0  1 0 |  1    1  | 1 |  1  | yes
                // 1 0  1 1 |  0    0  | 0 |  0  | no
                // 1 1  0 0 |  1    0  | 0 |  0  | no
                // 1 1  0 1 |  0    1  | 0 |  0  | no
                self.set_p_bit(StatusMask::Overflow, ((a ^ sum) & (!b ^ sum)) >> 7 != 0);
            }
            M::Sec => match opcode.mode {
                Implied => self.set_p_bit(StatusMask::Carry, true),
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            M::Sed => match opcode.mode {
                Implied => self.set_p_bit(StatusMask::Decimal, true),
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            M::Sei => match opcode.mode {
                Implied => self.set_p_bit(StatusMask::Interrupt, true),
                _ => panic!("illegal AddressMode: {:?}", opcode),
            },
            M::Sta => match self.read_operand(bus, opcode.mode) {
                OpValue::U16(addr) => bus.write(addr, self.a),
                _ => panic!("illegal AddressMode: {opcode:?}"),
            },
            M::Stx => match self.read_operand(bus, opcode.mode) {
                OpValue::U16(addr) => bus.write(addr, self.x),
                _ => panic!("illegal AddressMode: {opcode:?}"),
            },
            M::Sty => match self.read_operand(bus, opcode.mode) {
                OpValue::U16(addr) => bus.write(addr, self.y),
                _ => panic!("illegal AddressMode: {opcode:?}"),
            },
            M::Tax => {
                self.x = self.a;
                self.update_p_z_n(self.x);
            }
            M::Tay => {
                self.y = self.a;
                self.update_p_z_n(self.y);
            }
            M::Tsx => {
                self.x = self.s;
                self.update_p_z_n(self.x);
            }
            M::Txa => {
                self.a = self.x;
                self.update_p_z_n(self.a);
            }
            M::Txs => {
                self.s = self.x;
                self.update_p_z_n(self.s);
            }
            M::Tya => {
                self.a = self.y;
                self.update_p_z_n(self.a);
            }
        }
    }

    /// Read a u16 in little-endian order from the bus, crossing page boundaries.
    fn read_u16(&self, bus: &bus::Bus, addr: u16) -> u16 {
        let lo = bus.read(addr) as u16;
        let hi = bus.read(addr.wrapping_add(1)) as u16;
        hi << 8 | lo
    }

    /// Read a u16 in little-endian order from the bus, wrapping within a page.
    fn read_u16_zp(&self, bus: &bus::Bus, addr: u8) -> u16 {
        let lo = bus.read(addr as u16) as u16;
        let hi = bus.read(addr.wrapping_add(1) as u16) as u16;
        hi << 8 | lo
    }

    /// Read u8 from address pointed to by PC, incrementing PC
    fn read_pc_u8(&mut self, bus: &bus::Bus) -> u8 {
        let val = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        val
    }

    /// Read u16 from address pointed to by PC, incrementing PC
    fn read_pc_u16(&mut self, bus: &bus::Bus) -> u16 {
        let val = self.read_u16(bus, self.pc);
        self.pc = self.pc.wrapping_add(2);
        val
    }

    /// Reads the operand from bus, following indirection/indexing where necessary, returning
    /// an address or immediate value, and incrementing PC.
    fn read_operand(&mut self, bus: &bus::Bus, mode: isa::AddressMode) -> isa::OpValue {
        use isa::AddressMode::*;
        use isa::OpValue as OV;

        match mode {
            Absolute => OV::U16(self.read_pc_u16(bus)),
            AbsoluteX => OV::U16(self.read_pc_u16(bus).wrapping_add(self.x as u16)),
            AbsoluteY => OV::U16(self.read_pc_u16(bus).wrapping_add(self.y as u16)),
            Accumulator => OV::None,
            Immediate => OV::U8(self.read_pc_u8(bus)),
            Implied => OV::None,
            Indirect => {
                let ptr = self.read_pc_u16(bus);
                OV::U16(self.read_u16(bus, ptr))
            }
            IndirectY => {
                let ptr = self.read_pc_u8(bus);
                let addr = self.read_u16_zp(bus, ptr).wrapping_add(self.y as u16);
                OV::U16(addr)
            }
            Relative => {
                // #![feature(mixed_integer_ops)]
                // OV::U16(self.pc.wrapping_add_signed(self.read_pc_u8(bus).into()))
                let offset: i16 = (self.read_pc_u8(bus) as i8).into();
                let base = self.pc as i16;
                OV::U16((base + offset) as u16)
            }
            XIndirect => {
                let ptr = self.read_pc_u8(bus).wrapping_add(self.x);
                OV::U16(self.read_u16_zp(bus, ptr))
            }
            Zeropage => OV::U16(self.read_pc_u8(bus) as u16),
            ZeropageX => OV::U16(self.read_pc_u8(bus).wrapping_add(self.x) as u16),
            ZeropageY => OV::U16(self.read_pc_u8(bus).wrapping_add(self.y) as u16),
        }
    }

    fn read_operand_value(&mut self, bus: &bus::Bus, opcode: isa::Opcode) -> u8 {
        use isa::OpValue;
        match self.read_operand(bus, opcode.mode) {
            OpValue::U8(val) => val,
            OpValue::U16(addr) => bus.read(addr),
            OpValue::None => panic!("illegal AddressMode: {:?}", opcode),
        }
    }

    /// Update the Processor Status' Zero and Negative bits based on the specified value.
    fn update_p_z_n(&mut self, val: u8) {
        self.set_p_bit(StatusMask::Zero, val == 0);
        self.set_p_bit(StatusMask::Negative, (val as i8) < 0);
    }

    pub fn set_p_bit(&mut self, mask: StatusMask, val: bool) {
        let m = mask as u8;
        if val {
            self.p |= m
        } else {
            self.p &= !m
        }
    }

    pub fn get_p_bit(&mut self, mask: StatusMask) -> bool {
        self.p & mask as u8 != 0
    }

    fn carry(&mut self) -> u8 {
        let bit = StatusBit::Carry as u8;
        let mask = StatusMask::Carry as u8;
        (self.p & mask) >> bit
    }

    fn push_addr(&mut self, bus: &mut bus::Bus, addr: u16) {
        self.push(bus, (addr >> 8) as u8); // hi
        self.push(bus, addr as u8); // lo
    }

    fn push(&mut self, bus: &mut bus::Bus, val: u8) {
        bus.write(0x0100 | self.s as u16, val);
        self.s = self.s.wrapping_sub(1);
    }

    fn pop_addr(&mut self, bus: &bus::Bus) -> u16 {
        let lo = self.pop(bus) as u16;
        let hi = self.pop(bus) as u16;
        hi << 8 | lo
    }

    fn pop(&mut self, bus: &bus::Bus) -> u8 {
        self.s = self.s.wrapping_add(1);
        bus.read(0x0100 | self.s as u16)
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let stat = stat(&self.p);
        f.write_fmt(format_args!(
            "Cpu {{ P: {} PC: ${:04X} S: ${:02X} A: ${:02X} X: ${:02X} Y: ${:02X} }}",
            stat, self.pc, self.s, self.a, self.x, self.y,
        ))
    }
}

// a string representation of the 8-bit processor status register;
// enabled bits are upper case, disabled bits are lower case.
pub fn stat(p: &u8) -> String {
    "nv-bdizc"
        .chars()
        .enumerate()
        .map(|(i, x)| {
            if p >> (7 - i) & 1 == 1 {
                x.to_ascii_uppercase()
            } else {
                x
            }
        })
        .collect()
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

#[allow(unused)]
const VEC_NMI: u16 = 0xFFFA;
const VEC_RES: u16 = 0xFFFC;
const VEC_IRQ: u16 = 0xFFFE;

use std::collections::HashMap;
use std::fmt;

// Opcode is an 8-bit machine instruction alongside its Mnemonic and AddressMode.
#[derive(Copy, Clone)]
pub struct Opcode {
    pub code: u8,
    pub mnemonic: Mnemonic,
    pub mode: AddressMode,
}

impl Opcode {
    pub fn new(mnemonic: Mnemonic, mode: AddressMode, code: u8) -> Opcode {
        Opcode {
            code,
            mnemonic,
            mode,
        }
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Opcode {{ ${:02X} {} {:?} }}",
            self.code, self.mnemonic, self.mode
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Mnemonic {
    Adc, // add with carry
    And, // and (with accumulator)
    Asl, // arithmetic shift left
    Bcc, // branch on carry clear
    Bcs, // branch on carry set
    Beq, // branch on equal (zero set)
    Bit, // bit test
    Bmi, // branch on minus (negative set)
    Bne, // branch on not equal (zero clear)
    Bpl, // branch on plus (negative clear)
    Brk, // break / interrupt
    Bvc, // branch on overflow clear
    Bvs, // branch on overflow set
    Clc, // clear carry
    Cld, // clear decimal
    Cli, // clear interrupt disable
    Clv, // clear overflow
    Cmp, // compare (with accumulator)
    Cpx, // compare with X
    Cpy, // compare with Y
    Dec, // decrement
    Dex, // decrement X
    Dey, // decrement Y
    Eor, // exclusive or (with accumulator)
    Inc, // increment
    Inx, // increment X
    Iny, // increment Y
    Jmp, // jump
    Jsr, // jump subroutine
    Lda, // load accumulator
    Ldx, // load X
    Ldy, // load Y
    Lsr, // logical shift right
    Nop, // no operation
    Ora, // or with accumulator
    Pha, // push accumulator
    Php, // push processor status (SR)
    Pla, // pull accumulator
    Plp, // pull processor status (SR)
    Rol, // rotate left
    Ror, // rotate right
    Rti, // return from interrupt
    Rts, // return from subroutine
    Sbc, // subtract with carry
    Sec, // set carry
    Sed, // set decimal
    Sei, // set interrupt disable
    Sta, // store accumulator
    Stx, // store X
    Sty, // store Y
    Tax, // transfer accumulator to X
    Tay, // transfer accumulator to Y
    Tsx, // transfer stack pointer to X
    Txa, // transfer X to accumulator
    Txs, // transfer X to stack pointer
    Tya, // transfer Y to accumulator
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_uppercase())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AddressMode {
    Accumulator,
    Absolute,  // $LLHH
    AbsoluteX, // $LLHH,X
    AbsoluteY, // $LLHH,Y
    Immediate, // $BB
    Implied,
    Indirect,  // ($LLHH)
    XIndirect, // ($LL,X)
    IndirectY, // ($LL),Y
    Relative,  // $BB (signed)
    Zeropage,  // $LL
    ZeropageX, // $LL,X
    ZeropageY, // $LL,Y
}

/// Length in bytes of operand associated with given AddressMode
pub fn operand_length(mode: AddressMode) -> u16 {
    use AddressMode::*;
    match mode {
        Accumulator | Implied => 0,
        Immediate | XIndirect | IndirectY | Relative | Zeropage | ZeropageX | ZeropageY => 1,
        Absolute | AbsoluteX | AbsoluteY | Indirect => 2,
    }
}

// OpValue represents the resolved value of an Opcode operand, after indirection, indexing etc.
#[derive(Debug)]
pub enum OpValue {
    None,
    U8(u8),
    U16(u16),
}

// All supported Opcodes
pub fn opcode_list() -> Vec<Opcode> {
    use AddressMode::*; // Absolute, Immediate etc
    use Mnemonic::*; // Adc, Bcc, Clc etc
    let new = Opcode::new;
    vec![
        new(Adc, Absolute, 0x6D),
        new(Adc, AbsoluteX, 0x7D),
        new(Adc, AbsoluteY, 0x79),
        new(Adc, Immediate, 0x69),
        new(Adc, IndirectY, 0x71),
        new(Adc, XIndirect, 0x61),
        new(Adc, Zeropage, 0x65),
        new(Adc, ZeropageX, 0x75),
        new(And, Absolute, 0x2D),
        new(And, AbsoluteX, 0x3D),
        new(And, AbsoluteY, 0x39),
        new(And, Immediate, 0x29),
        new(And, IndirectY, 0x31),
        new(And, XIndirect, 0x21),
        new(And, Zeropage, 0x25),
        new(And, ZeropageX, 0x35),
        new(Asl, Absolute, 0x0E),
        new(Asl, AbsoluteX, 0x1E),
        new(Asl, Accumulator, 0x0A),
        new(Asl, Zeropage, 0x06),
        new(Asl, ZeropageX, 0x16),
        new(Bcc, Relative, 0x90),
        new(Bcs, Relative, 0xB0),
        new(Beq, Relative, 0xF0),
        new(Bit, Absolute, 0x2C),
        new(Bit, Zeropage, 0x24),
        new(Bmi, Relative, 0x30),
        new(Bne, Relative, 0xD0),
        new(Bpl, Relative, 0x10),
        new(Brk, Implied, 0x00),
        new(Bvc, Relative, 0x50),
        new(Bvs, Relative, 0x70),
        new(Clc, Implied, 0x18),
        new(Cld, Implied, 0xD8),
        new(Cli, Implied, 0x58),
        new(Clv, Implied, 0xB8),
        new(Cmp, Absolute, 0xCD),
        new(Cmp, AbsoluteX, 0xDD),
        new(Cmp, AbsoluteY, 0xD9),
        new(Cmp, Immediate, 0xC9),
        new(Cmp, IndirectY, 0xD1),
        new(Cmp, XIndirect, 0xC1),
        new(Cmp, Zeropage, 0xC5),
        new(Cmp, ZeropageX, 0xD5),
        new(Cpx, Absolute, 0xEC),
        new(Cpx, Immediate, 0xE0),
        new(Cpx, Zeropage, 0xE4),
        new(Cpy, Absolute, 0xCC),
        new(Cpy, Immediate, 0xC0),
        new(Cpy, Zeropage, 0xC4),
        new(Dec, Absolute, 0xCE),
        new(Dec, AbsoluteX, 0xDE),
        new(Dec, Zeropage, 0xC6),
        new(Dec, ZeropageX, 0xD6),
        new(Dex, Implied, 0xCA),
        new(Dey, Implied, 0x88),
        new(Eor, Absolute, 0x4D),
        new(Eor, AbsoluteX, 0x5D),
        new(Eor, AbsoluteY, 0x59),
        new(Eor, Immediate, 0x49),
        new(Eor, IndirectY, 0x51),
        new(Eor, XIndirect, 0x41),
        new(Eor, Zeropage, 0x45),
        new(Eor, ZeropageX, 0x55),
        new(Inc, Absolute, 0xEE),
        new(Inc, AbsoluteX, 0xFE),
        new(Inc, Zeropage, 0xE6),
        new(Inc, ZeropageX, 0xF6),
        new(Inx, Implied, 0xE8),
        new(Iny, Implied, 0xC8),
        new(Jmp, Absolute, 0x4C),
        new(Jmp, Indirect, 0x6C),
        new(Jsr, Absolute, 0x20),
        new(Lda, Absolute, 0xAD),
        new(Lda, AbsoluteX, 0xBD),
        new(Lda, AbsoluteY, 0xB9),
        new(Lda, Immediate, 0xA9),
        new(Lda, IndirectY, 0xB1),
        new(Lda, XIndirect, 0xA1),
        new(Lda, Zeropage, 0xA5),
        new(Lda, ZeropageX, 0xB5),
        new(Ldx, Absolute, 0xAE),
        new(Ldx, AbsoluteY, 0xBE),
        new(Ldx, Immediate, 0xA2),
        new(Ldx, Zeropage, 0xA6),
        new(Ldx, ZeropageY, 0xB6),
        new(Ldy, Absolute, 0xAC),
        new(Ldy, AbsoluteX, 0xBC),
        new(Ldy, Immediate, 0xA0),
        new(Ldy, Zeropage, 0xA4),
        new(Ldy, ZeropageX, 0xB4),
        new(Lsr, Absolute, 0x4E),
        new(Lsr, AbsoluteX, 0x5E),
        new(Lsr, Accumulator, 0x4A),
        new(Lsr, Zeropage, 0x46),
        new(Lsr, ZeropageX, 0x56),
        new(Nop, Implied, 0xEA),
        new(Ora, Absolute, 0x0D),
        new(Ora, AbsoluteX, 0x1D),
        new(Ora, AbsoluteY, 0x19),
        new(Ora, Immediate, 0x09),
        new(Ora, IndirectY, 0x11),
        new(Ora, XIndirect, 0x01),
        new(Ora, Zeropage, 0x05),
        new(Ora, ZeropageX, 0x15),
        new(Pha, Implied, 0x48),
        new(Php, Implied, 0x08),
        new(Pla, Implied, 0x68),
        new(Plp, Implied, 0x28),
        new(Rol, Absolute, 0x2E),
        new(Rol, AbsoluteX, 0x3E),
        new(Rol, Accumulator, 0x2A),
        new(Rol, Zeropage, 0x26),
        new(Rol, ZeropageX, 0x36),
        new(Ror, Absolute, 0x6E),
        new(Ror, AbsoluteX, 0x7E),
        new(Ror, Accumulator, 0x6A),
        new(Ror, Zeropage, 0x66),
        new(Ror, ZeropageX, 0x76),
        new(Rti, Implied, 0x40),
        new(Rts, Implied, 0x60),
        new(Sbc, Absolute, 0xED),
        new(Sbc, AbsoluteX, 0xFD),
        new(Sbc, AbsoluteY, 0xF9),
        new(Sbc, Immediate, 0xE9),
        new(Sbc, IndirectY, 0xF1),
        new(Sbc, XIndirect, 0xE1),
        new(Sbc, Zeropage, 0xE5),
        new(Sbc, ZeropageX, 0xF5),
        new(Sec, Implied, 0x38),
        new(Sed, Implied, 0xF8),
        new(Sei, Implied, 0x78),
        new(Sta, Absolute, 0x8D),
        new(Sta, AbsoluteX, 0x9D),
        new(Sta, AbsoluteY, 0x99),
        new(Sta, IndirectY, 0x91),
        new(Sta, XIndirect, 0x81),
        new(Sta, Zeropage, 0x85),
        new(Sta, ZeropageX, 0x95),
        new(Stx, Absolute, 0x8E),
        new(Stx, Zeropage, 0x86),
        new(Stx, ZeropageY, 0x96),
        new(Sty, Absolute, 0x8C),
        new(Sty, Zeropage, 0x84),
        new(Sty, ZeropageX, 0x94),
        new(Tax, Implied, 0xAA),
        new(Tay, Implied, 0xA8),
        new(Tsx, Implied, 0xBA),
        new(Txa, Implied, 0x8A),
        new(Txs, Implied, 0x9A),
        new(Tya, Implied, 0x98),
    ]
}

pub struct OpcodeByMnemonicAndAddressMode {
    map: HashMap<Mnemonic, HashMap<AddressMode, Opcode>>,
}

impl OpcodeByMnemonicAndAddressMode {
    pub fn build() -> Self {
        let mut map: HashMap<_, HashMap<_, _>> = HashMap::new();
        for op in opcode_list() {
            map.entry(op.mnemonic).or_default().insert(op.mode, op);
        }
        Self { map }
    }

    pub fn get(&self, m: Mnemonic, am: AddressMode) -> Result<Opcode, Error> {
        self.map
            .get(&m)
            .unwrap() // all Mnemonic values should be in the HashMap
            .get(&am) // might be None for this AddressMode
            .copied() // Option<&Opcode> -> Option<Opcode>
            .ok_or(Error::IllegalAddressMode(m, am))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    IllegalAddressMode(Mnemonic, AddressMode),
}

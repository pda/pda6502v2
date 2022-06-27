use std::fmt;

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
            "Opcode {{ ${:02X} {:?} {:?} }}",
            self.code, self.mnemonic, self.mode
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Mnemonic {
    ADC, // add with carry
    AND, // and (with accumulator)
    ASL, // arithmetic shift left
    BCC, // branch on carry clear
    BCS, // branch on carry set
    BEQ, // branch on equal (zero set)
    BIT, // bit test
    BMI, // branch on minus (negative set)
    BNE, // branch on not equal (zero clear)
    BPL, // branch on plus (negative clear)
    BRK, // break / interrupt
    BVC, // branch on overflow clear
    BVS, // branch on overflow set
    CLC, // clear carry
    CLD, // clear decimal
    CLI, // clear interrupt disable
    CLV, // clear overflow
    CMP, // compare (with accumulator)
    CPX, // compare with X
    CPY, // compare with Y
    DEC, // decrement
    DEX, // decrement X
    DEY, // decrement Y
    EOR, // exclusive or (with accumulator)
    INC, // increment
    INX, // increment X
    INY, // increment Y
    JMP, // jump
    JSR, // jump subroutine
    LDA, // load accumulator
    LDX, // load X
    LDY, // load Y
    LSR, // logical shift right
    NOP, // no operation
    ORA, // or with accumulator
    PHA, // push accumulator
    PHP, // push processor status (SR)
    PLA, // pull accumulator
    PLP, // pull processor status (SR)
    ROL, // rotate left
    ROR, // rotate right
    RTI, // return from interrupt
    RTS, // return from subroutine
    SBC, // subtract with carry
    SEC, // set carry
    SED, // set decimal
    SEI, // set interrupt disable
    STA, // store accumulator
    STX, // store X
    STY, // store Y
    TAX, // transfer accumulator to X
    TAY, // transfer accumulator to Y
    TSX, // transfer stack pointer to X
    TXA, // transfer X to accumulator
    TXS, // transfer X to stack pointer
    TYA, // transfer Y to accumulator
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

// All supported Opcodes
pub fn opcode_list() -> Vec<Opcode> {
    use AddressMode::*; // Absolute, Immediate etc
    use Mnemonic::*; // ADC, AND etc
    let new = Opcode::new;
    vec![
        new(ADC, Absolute, 0x6D),
        new(ADC, AbsoluteX, 0x7D),
        new(ADC, AbsoluteY, 0x79),
        new(ADC, Immediate, 0x69),
        new(ADC, IndirectY, 0x71),
        new(ADC, XIndirect, 0x61),
        new(ADC, Zeropage, 0x65),
        new(ADC, ZeropageX, 0x75),
        new(AND, Absolute, 0x2D),
        new(AND, AbsoluteX, 0x3D),
        new(AND, AbsoluteY, 0x39),
        new(AND, Immediate, 0x29),
        new(AND, IndirectY, 0x31),
        new(AND, XIndirect, 0x21),
        new(AND, Zeropage, 0x25),
        new(AND, ZeropageX, 0x35),
        new(ASL, Absolute, 0x0E),
        new(ASL, AbsoluteX, 0x1E),
        new(ASL, Accumulator, 0x0A),
        new(ASL, Zeropage, 0x06),
        new(ASL, ZeropageX, 0x16),
        new(BCC, Relative, 0x90),
        new(BCS, Relative, 0xB0),
        new(BEQ, Relative, 0xF0),
        new(BIT, Absolute, 0x2C),
        new(BIT, Zeropage, 0x24),
        new(BMI, Relative, 0x30),
        new(BNE, Relative, 0xD0),
        new(BPL, Relative, 0x10),
        new(BRK, Implied, 0x00),
        new(BVC, Relative, 0x50),
        new(BVS, Relative, 0x70),
        new(CLC, Implied, 0x18),
        new(CLD, Implied, 0xD8),
        new(CLI, Implied, 0x58),
        new(CLV, Implied, 0xB8),
        new(CMP, Absolute, 0xCD),
        new(CMP, AbsoluteX, 0xDD),
        new(CMP, AbsoluteY, 0xD9),
        new(CMP, Immediate, 0xC9),
        new(CMP, IndirectY, 0xD1),
        new(CMP, XIndirect, 0xC1),
        new(CMP, Zeropage, 0xC5),
        new(CMP, ZeropageX, 0xD5),
        new(CPX, Absolute, 0xEC),
        new(CPX, Immediate, 0xE0),
        new(CPX, Zeropage, 0xE4),
        new(CPY, Absolute, 0xCC),
        new(CPY, Immediate, 0xC0),
        new(CPY, Zeropage, 0xC4),
        new(DEC, Absolute, 0xCE),
        new(DEC, AbsoluteX, 0xDE),
        new(DEC, Zeropage, 0xC6),
        new(DEC, ZeropageX, 0xD6),
        new(DEX, Implied, 0xCA),
        new(DEY, Implied, 0x88),
        new(EOR, Absolute, 0x4D),
        new(EOR, AbsoluteX, 0x5D),
        new(EOR, AbsoluteY, 0x59),
        new(EOR, Immediate, 0x49),
        new(EOR, IndirectY, 0x51),
        new(EOR, XIndirect, 0x41),
        new(EOR, Zeropage, 0x45),
        new(EOR, ZeropageX, 0x55),
        new(INC, Absolute, 0xEE),
        new(INC, AbsoluteX, 0xFE),
        new(INC, Zeropage, 0xE6),
        new(INC, ZeropageX, 0xF6),
        new(INX, Implied, 0xE8),
        new(INY, Implied, 0xC8),
        new(JMP, Absolute, 0x4C),
        new(JMP, Indirect, 0x6C),
        new(JSR, Absolute, 0x20),
        new(LDA, Absolute, 0xAD),
        new(LDA, AbsoluteX, 0xBD),
        new(LDA, AbsoluteY, 0xB9),
        new(LDA, Immediate, 0xA9),
        new(LDA, IndirectY, 0xB1),
        new(LDA, XIndirect, 0xA1),
        new(LDA, Zeropage, 0xA5),
        new(LDA, ZeropageX, 0xB5),
        new(LDX, Absolute, 0xAE),
        new(LDX, AbsoluteY, 0xBE),
        new(LDX, Immediate, 0xA2),
        new(LDX, Zeropage, 0xA6),
        new(LDX, ZeropageY, 0xB6),
        new(LDY, Absolute, 0xAC),
        new(LDY, AbsoluteX, 0xBC),
        new(LDY, Immediate, 0xA0),
        new(LDY, Zeropage, 0xA4),
        new(LDY, ZeropageX, 0xB4),
        new(LSR, Absolute, 0x4E),
        new(LSR, AbsoluteX, 0x5E),
        new(LSR, Accumulator, 0x4A),
        new(LSR, Zeropage, 0x46),
        new(LSR, ZeropageX, 0x56),
        new(NOP, Implied, 0xEA),
        new(ORA, Absolute, 0x0D),
        new(ORA, AbsoluteX, 0x1D),
        new(ORA, AbsoluteY, 0x19),
        new(ORA, Immediate, 0x09),
        new(ORA, IndirectY, 0x11),
        new(ORA, XIndirect, 0x01),
        new(ORA, Zeropage, 0x05),
        new(ORA, ZeropageX, 0x15),
        new(PHA, Implied, 0x48),
        new(PHP, Implied, 0x08),
        new(PLA, Implied, 0x68),
        new(PLP, Implied, 0x28),
        new(ROL, Absolute, 0x2E),
        new(ROL, AbsoluteX, 0x3E),
        new(ROL, Accumulator, 0x2A),
        new(ROL, Zeropage, 0x26),
        new(ROL, ZeropageX, 0x36),
        new(ROR, Absolute, 0x6E),
        new(ROR, AbsoluteX, 0x7E),
        new(ROR, Accumulator, 0x6A),
        new(ROR, Zeropage, 0x66),
        new(ROR, ZeropageX, 0x76),
        new(RTI, Implied, 0x40),
        new(RTS, Implied, 0x60),
        new(SBC, Absolute, 0xED),
        new(SBC, AbsoluteX, 0xFD),
        new(SBC, AbsoluteY, 0xF9),
        new(SBC, Immediate, 0xE9),
        new(SBC, IndirectY, 0xF1),
        new(SBC, XIndirect, 0xE1),
        new(SBC, Zeropage, 0xE5),
        new(SBC, ZeropageX, 0xF5),
        new(SEC, Implied, 0x38),
        new(SED, Implied, 0xF8),
        new(SEI, Implied, 0x78),
        new(STA, Absolute, 0x8D),
        new(STA, AbsoluteX, 0x9D),
        new(STA, AbsoluteY, 0x99),
        new(STA, IndirectY, 0x91),
        new(STA, XIndirect, 0x81),
        new(STA, Zeropage, 0x85),
        new(STA, ZeropageX, 0x95),
        new(STX, Absolute, 0x8E),
        new(STX, Zeropage, 0x86),
        new(STX, ZeropageY, 0x96),
        new(STY, Absolute, 0x8C),
        new(STY, Zeropage, 0x84),
        new(STY, ZeropageX, 0x94),
        new(TAX, Implied, 0xAA),
        new(TAY, Implied, 0xA8),
        new(TSX, Implied, 0xBA),
        new(TXA, Implied, 0x8A),
        new(TXS, Implied, 0x9A),
        new(TYA, Implied, 0x98),
    ]
}

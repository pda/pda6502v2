use crate::bus::Bus;
use core::fmt::Debug;
use core::fmt::Formatter;

pub const OP_NOP: u8 = 0xEA;

pub struct Cpu {
    bus: Bus,
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,
    sr: u8,
    optab: [Option<Instruction>; 256],
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
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

    pub fn reset(&mut self) {
        self.pc = self.bus.read16(0xFFFC); // TODO: load vector
        self.sp = 0x00;
        self.a = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        self.sr = 0x00;
    }

    pub fn step(&mut self) {
        let opcode = self.bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        match self.optab[opcode as usize] {
            None => panic!("illegal opcode"),
            Some(instruction) => self.execute(instruction),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        println!("{:?}", instruction);
        match instruction.mnemonic {
            // Mnemonic::ADC => {}
            // Mnemonic::AND => {}
            // Mnemonic::ASL => {}
            // Mnemonic::BCC => {}
            // Mnemonic::BCS => {}
            // Mnemonic::BEQ => {}
            // Mnemonic::BIT => {}
            // Mnemonic::BMI => {}
            // Mnemonic::BNE => {}
            // Mnemonic::BPL => {}
            // Mnemonic::BRK => {}
            // Mnemonic::BVC => {}
            // Mnemonic::BVS => {}
            // Mnemonic::CLC => {}
            // Mnemonic::CLD => {}
            // Mnemonic::CLI => {}
            // Mnemonic::CLV => {}
            // Mnemonic::CMP => {}
            // Mnemonic::CPX => {}
            // Mnemonic::CPY => {}
            // Mnemonic::DEC => {}
            // Mnemonic::DEX => {}
            // Mnemonic::DEY => {}
            // Mnemonic::EOR => {}
            // Mnemonic::INC => {}
            // Mnemonic::INX => {}
            // Mnemonic::INY => {}
            // Mnemonic::JMP => {}
            // Mnemonic::JSR => {}
            // Mnemonic::LDA => {}
            // Mnemonic::LDX => {}
            // Mnemonic::LDY => {}
            // Mnemonic::LSR => {}
            Mnemonic::NOP => {}
            // Mnemonic::ORA => {}
            // Mnemonic::PHA => {}
            // Mnemonic::PHP => {}
            // Mnemonic::PLA => {}
            // Mnemonic::PLP => {}
            // Mnemonic::ROL => {}
            // Mnemonic::ROR => {}
            // Mnemonic::RTI => {}
            // Mnemonic::RTS => {}
            // Mnemonic::SBC => {}
            // Mnemonic::SEC => {}
            // Mnemonic::SED => {}
            // Mnemonic::SEI => {}
            // Mnemonic::STA => {}
            // Mnemonic::STX => {}
            // Mnemonic::STY => {}
            // Mnemonic::TAX => {}
            // Mnemonic::TAY => {}
            // Mnemonic::TSX => {}
            // Mnemonic::TXA => {}
            // Mnemonic::TXS => {}
            // Mnemonic::TYA => {}
            x => todo!("{:?}", x),
        }
    }
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let stat: String = "nv-bdizc"
            .chars()
            .enumerate()
            .map(|(i, x)| {
                if self.sr >> (7 - i) & 1 == 1 {
                    x.to_ascii_uppercase()
                } else {
                    x
                }
            })
            .collect();
        f.write_fmt(format_args!(
            "Cpu {{ SR: {} PC: ${:04X} SP: ${:02X} A: ${:02X} X: ${:02X} Y: ${:02X} }}",
            stat, self.pc, self.sp, self.a, self.x, self.y,
        ))
    }
}

fn build_opcode_table() -> [Option<Instruction>; 256] {
    let oplist = vec![
        Instruction::new(Mnemonic::ADC, AddressMode::Absolute, 0x6D),
        Instruction::new(Mnemonic::ADC, AddressMode::AbsoluteX, 0x7D),
        Instruction::new(Mnemonic::ADC, AddressMode::AbsoluteY, 0x79),
        Instruction::new(Mnemonic::ADC, AddressMode::Immediate, 0x69),
        Instruction::new(Mnemonic::ADC, AddressMode::IndirectY, 0x71),
        Instruction::new(Mnemonic::ADC, AddressMode::XIndirect, 0x61),
        Instruction::new(Mnemonic::ADC, AddressMode::Zeropage, 0x65),
        Instruction::new(Mnemonic::ADC, AddressMode::ZeropageX, 0x75),
        Instruction::new(Mnemonic::AND, AddressMode::Absolute, 0x2D),
        Instruction::new(Mnemonic::AND, AddressMode::AbsoluteX, 0x3D),
        Instruction::new(Mnemonic::AND, AddressMode::AbsoluteY, 0x39),
        Instruction::new(Mnemonic::AND, AddressMode::Immediate, 0x29),
        Instruction::new(Mnemonic::AND, AddressMode::IndirectY, 0x31),
        Instruction::new(Mnemonic::AND, AddressMode::XIndirect, 0x21),
        Instruction::new(Mnemonic::AND, AddressMode::Zeropage, 0x25),
        Instruction::new(Mnemonic::AND, AddressMode::ZeropageX, 0x35),
        Instruction::new(Mnemonic::ASL, AddressMode::Absolute, 0x0E),
        Instruction::new(Mnemonic::ASL, AddressMode::AbsoluteX, 0x1E),
        Instruction::new(Mnemonic::ASL, AddressMode::Accumulator, 0x0A),
        Instruction::new(Mnemonic::ASL, AddressMode::Zeropage, 0x06),
        Instruction::new(Mnemonic::ASL, AddressMode::ZeropageX, 0x16),
        Instruction::new(Mnemonic::BCC, AddressMode::Relative, 0x90),
        Instruction::new(Mnemonic::BCS, AddressMode::Relative, 0xB0),
        Instruction::new(Mnemonic::BEQ, AddressMode::Relative, 0xF0),
        Instruction::new(Mnemonic::BIT, AddressMode::Absolute, 0x2C),
        Instruction::new(Mnemonic::BIT, AddressMode::Zeropage, 0x24),
        Instruction::new(Mnemonic::BMI, AddressMode::Relative, 0x30),
        Instruction::new(Mnemonic::BNE, AddressMode::Relative, 0xD0),
        Instruction::new(Mnemonic::BPL, AddressMode::Relative, 0x10),
        Instruction::new(Mnemonic::BRK, AddressMode::Implied, 0x00),
        Instruction::new(Mnemonic::BVC, AddressMode::Relative, 0x50),
        Instruction::new(Mnemonic::BVS, AddressMode::Relative, 0x70),
        Instruction::new(Mnemonic::CLC, AddressMode::Implied, 0x18),
        Instruction::new(Mnemonic::CLD, AddressMode::Implied, 0xD8),
        Instruction::new(Mnemonic::CLI, AddressMode::Implied, 0x58),
        Instruction::new(Mnemonic::CLV, AddressMode::Implied, 0xB8),
        Instruction::new(Mnemonic::CMP, AddressMode::Absolute, 0xCD),
        Instruction::new(Mnemonic::CMP, AddressMode::AbsoluteX, 0xDD),
        Instruction::new(Mnemonic::CMP, AddressMode::AbsoluteY, 0xD9),
        Instruction::new(Mnemonic::CMP, AddressMode::Immediate, 0xC9),
        Instruction::new(Mnemonic::CMP, AddressMode::IndirectY, 0xD1),
        Instruction::new(Mnemonic::CMP, AddressMode::XIndirect, 0xC1),
        Instruction::new(Mnemonic::CMP, AddressMode::Zeropage, 0xC5),
        Instruction::new(Mnemonic::CMP, AddressMode::ZeropageX, 0xD5),
        Instruction::new(Mnemonic::CPX, AddressMode::Absolute, 0xEC),
        Instruction::new(Mnemonic::CPX, AddressMode::Immediate, 0xE0),
        Instruction::new(Mnemonic::CPX, AddressMode::Zeropage, 0xE4),
        Instruction::new(Mnemonic::CPY, AddressMode::Absolute, 0xCC),
        Instruction::new(Mnemonic::CPY, AddressMode::Immediate, 0xC0),
        Instruction::new(Mnemonic::CPY, AddressMode::Zeropage, 0xC4),
        Instruction::new(Mnemonic::DEC, AddressMode::Absolute, 0xCE),
        Instruction::new(Mnemonic::DEC, AddressMode::AbsoluteX, 0xDE),
        Instruction::new(Mnemonic::DEC, AddressMode::Zeropage, 0xC6),
        Instruction::new(Mnemonic::DEC, AddressMode::ZeropageX, 0xD6),
        Instruction::new(Mnemonic::DEX, AddressMode::Implied, 0xCA),
        Instruction::new(Mnemonic::DEY, AddressMode::Implied, 0x88),
        Instruction::new(Mnemonic::EOR, AddressMode::Absolute, 0x4D),
        Instruction::new(Mnemonic::EOR, AddressMode::AbsoluteX, 0x5D),
        Instruction::new(Mnemonic::EOR, AddressMode::AbsoluteY, 0x59),
        Instruction::new(Mnemonic::EOR, AddressMode::Immediate, 0x49),
        Instruction::new(Mnemonic::EOR, AddressMode::IndirectY, 0x51),
        Instruction::new(Mnemonic::EOR, AddressMode::XIndirect, 0x41),
        Instruction::new(Mnemonic::EOR, AddressMode::Zeropage, 0x45),
        Instruction::new(Mnemonic::EOR, AddressMode::ZeropageX, 0x55),
        Instruction::new(Mnemonic::INC, AddressMode::Absolute, 0xEE),
        Instruction::new(Mnemonic::INC, AddressMode::AbsoluteX, 0xFE),
        Instruction::new(Mnemonic::INC, AddressMode::Zeropage, 0xE6),
        Instruction::new(Mnemonic::INC, AddressMode::ZeropageX, 0xF6),
        Instruction::new(Mnemonic::INX, AddressMode::Implied, 0xE8),
        Instruction::new(Mnemonic::INY, AddressMode::Implied, 0xC8),
        Instruction::new(Mnemonic::JMP, AddressMode::Absolute, 0x4C),
        Instruction::new(Mnemonic::JMP, AddressMode::Indirect, 0x6C),
        Instruction::new(Mnemonic::JSR, AddressMode::Absolute, 0x20),
        Instruction::new(Mnemonic::LDA, AddressMode::Absolute, 0xAD),
        Instruction::new(Mnemonic::LDA, AddressMode::AbsoluteX, 0xBD),
        Instruction::new(Mnemonic::LDA, AddressMode::AbsoluteY, 0xB9),
        Instruction::new(Mnemonic::LDA, AddressMode::Immediate, 0xA9),
        Instruction::new(Mnemonic::LDA, AddressMode::IndirectY, 0xB1),
        Instruction::new(Mnemonic::LDA, AddressMode::XIndirect, 0xA1),
        Instruction::new(Mnemonic::LDA, AddressMode::Zeropage, 0xA5),
        Instruction::new(Mnemonic::LDA, AddressMode::ZeropageX, 0xB5),
        Instruction::new(Mnemonic::LDX, AddressMode::Absolute, 0xAE),
        Instruction::new(Mnemonic::LDX, AddressMode::AbsoluteY, 0xBE),
        Instruction::new(Mnemonic::LDX, AddressMode::Immediate, 0xA2),
        Instruction::new(Mnemonic::LDX, AddressMode::Zeropage, 0xA6),
        Instruction::new(Mnemonic::LDX, AddressMode::ZeropageY, 0xB6),
        Instruction::new(Mnemonic::LDY, AddressMode::Absolute, 0xAC),
        Instruction::new(Mnemonic::LDY, AddressMode::AbsoluteX, 0xBC),
        Instruction::new(Mnemonic::LDY, AddressMode::Immediate, 0xA0),
        Instruction::new(Mnemonic::LDY, AddressMode::Zeropage, 0xA4),
        Instruction::new(Mnemonic::LDY, AddressMode::ZeropageX, 0xB4),
        Instruction::new(Mnemonic::LSR, AddressMode::Absolute, 0x4E),
        Instruction::new(Mnemonic::LSR, AddressMode::AbsoluteX, 0x5E),
        Instruction::new(Mnemonic::LSR, AddressMode::Accumulator, 0x4A),
        Instruction::new(Mnemonic::LSR, AddressMode::Zeropage, 0x46),
        Instruction::new(Mnemonic::LSR, AddressMode::ZeropageX, 0x56),
        Instruction::new(Mnemonic::NOP, AddressMode::Implied, 0xEA),
        Instruction::new(Mnemonic::ORA, AddressMode::Absolute, 0x0D),
        Instruction::new(Mnemonic::ORA, AddressMode::AbsoluteX, 0x1D),
        Instruction::new(Mnemonic::ORA, AddressMode::AbsoluteY, 0x19),
        Instruction::new(Mnemonic::ORA, AddressMode::Immediate, 0x09),
        Instruction::new(Mnemonic::ORA, AddressMode::IndirectY, 0x11),
        Instruction::new(Mnemonic::ORA, AddressMode::XIndirect, 0x01),
        Instruction::new(Mnemonic::ORA, AddressMode::Zeropage, 0x05),
        Instruction::new(Mnemonic::ORA, AddressMode::ZeropageX, 0x15),
        Instruction::new(Mnemonic::PHA, AddressMode::Implied, 0x48),
        Instruction::new(Mnemonic::PHP, AddressMode::Implied, 0x08),
        Instruction::new(Mnemonic::PLA, AddressMode::Implied, 0x68),
        Instruction::new(Mnemonic::PLP, AddressMode::Implied, 0x28),
        Instruction::new(Mnemonic::ROL, AddressMode::Absolute, 0x2E),
        Instruction::new(Mnemonic::ROL, AddressMode::AbsoluteX, 0x3E),
        Instruction::new(Mnemonic::ROL, AddressMode::Accumulator, 0x2A),
        Instruction::new(Mnemonic::ROL, AddressMode::Zeropage, 0x26),
        Instruction::new(Mnemonic::ROL, AddressMode::ZeropageX, 0x36),
        Instruction::new(Mnemonic::ROR, AddressMode::Absolute, 0x6E),
        Instruction::new(Mnemonic::ROR, AddressMode::AbsoluteX, 0x7E),
        Instruction::new(Mnemonic::ROR, AddressMode::Accumulator, 0x6A),
        Instruction::new(Mnemonic::ROR, AddressMode::Zeropage, 0x66),
        Instruction::new(Mnemonic::ROR, AddressMode::ZeropageX, 0x76),
        Instruction::new(Mnemonic::RTI, AddressMode::Implied, 0x40),
        Instruction::new(Mnemonic::RTS, AddressMode::Implied, 0x60),
        Instruction::new(Mnemonic::SBC, AddressMode::Absolute, 0xED),
        Instruction::new(Mnemonic::SBC, AddressMode::AbsoluteX, 0xFD),
        Instruction::new(Mnemonic::SBC, AddressMode::AbsoluteY, 0xF9),
        Instruction::new(Mnemonic::SBC, AddressMode::Immediate, 0xE9),
        Instruction::new(Mnemonic::SBC, AddressMode::IndirectY, 0xF1),
        Instruction::new(Mnemonic::SBC, AddressMode::XIndirect, 0xE1),
        Instruction::new(Mnemonic::SBC, AddressMode::Zeropage, 0xE5),
        Instruction::new(Mnemonic::SBC, AddressMode::ZeropageX, 0xF5),
        Instruction::new(Mnemonic::SEC, AddressMode::Implied, 0x38),
        Instruction::new(Mnemonic::SED, AddressMode::Implied, 0xF8),
        Instruction::new(Mnemonic::SEI, AddressMode::Implied, 0x78),
        Instruction::new(Mnemonic::STA, AddressMode::Absolute, 0x8D),
        Instruction::new(Mnemonic::STA, AddressMode::AbsoluteX, 0x9D),
        Instruction::new(Mnemonic::STA, AddressMode::AbsoluteY, 0x99),
        Instruction::new(Mnemonic::STA, AddressMode::IndirectY, 0x91),
        Instruction::new(Mnemonic::STA, AddressMode::XIndirect, 0x81),
        Instruction::new(Mnemonic::STA, AddressMode::Zeropage, 0x85),
        Instruction::new(Mnemonic::STA, AddressMode::ZeropageX, 0x95),
        Instruction::new(Mnemonic::STX, AddressMode::Absolute, 0x8E),
        Instruction::new(Mnemonic::STX, AddressMode::Zeropage, 0x86),
        Instruction::new(Mnemonic::STX, AddressMode::ZeropageY, 0x96),
        Instruction::new(Mnemonic::STY, AddressMode::Absolute, 0x8C),
        Instruction::new(Mnemonic::STY, AddressMode::Zeropage, 0x84),
        Instruction::new(Mnemonic::STY, AddressMode::ZeropageX, 0x94),
        Instruction::new(Mnemonic::TAX, AddressMode::Implied, 0xAA),
        Instruction::new(Mnemonic::TAY, AddressMode::Implied, 0xA8),
        Instruction::new(Mnemonic::TSX, AddressMode::Implied, 0xBA),
        Instruction::new(Mnemonic::TXA, AddressMode::Implied, 0x8A),
        Instruction::new(Mnemonic::TXS, AddressMode::Implied, 0x9A),
        Instruction::new(Mnemonic::TYA, AddressMode::Implied, 0x98),
    ];
    let mut optab = [None; 256];
    for opcode in oplist {
        let code = opcode.code as usize;
        optab[code] = Some(opcode);
    }
    return optab;
}

#[derive(Copy, Clone)]
struct Instruction {
    code: u8,
    mnemonic: Mnemonic,
    mode: AddressMode,
}

impl Instruction {
    fn new(mnemonic: Mnemonic, mode: AddressMode, code: u8) -> Instruction {
        Instruction {
            code,
            mnemonic,
            mode,
        }
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instruction")
            .field("code", &format_args!("${:02X}", &self.code))
            .field("mnemonic", &self.mnemonic)
            .field("mode", &self.mode)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
enum Mnemonic {
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

#[derive(Copy, Clone, Debug)]
enum AddressMode {
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

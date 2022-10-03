use std::collections::HashMap;
use std::fmt;

use crate::isa;
use crate::isa::{AddressMode, Mnemonic, OpValue, Opcode};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_assembles_single_nop() {
        let mut asm = Assembler::new();
        assert_eq!(asm.nop().assemble().unwrap(), vec![0xEA]);
    }

    #[test]
    fn it_assembles_tiny_program_with_absolute_operand() {
        let mut asm = Assembler::new();
        asm.nop().nop().jmp(Operand::Abs(Addr::Literal(0x1234)));
        println!("{}", asm);
        assert_eq!(asm.assemble().unwrap(), vec![0xEA, 0xEA, 0x4C, 0x34, 0x12]);
    }

    #[test]
    fn it_assembles_labels() {
        let mut asm = Assembler::new();
        asm.org(0x1000)
            .label("foo")
            .jmp(Operand::Abs(label("bar")))
            .label("bar")
            .jmp(Operand::Abs(label("foo")));
        println!("{}", asm);
        assert_eq!(
            asm.assemble().unwrap(),
            vec![0x4C, 0x03, 0x10, 0x4C, 0x00, 0x10]
        );
    }

    #[test]
    #[should_panic(expected = "IllegalAddressMode(Jmp, Relative)")]
    fn it_errors_on_illegal_address_mode() {
        let mut asm = Assembler::new();
        asm.jmp(Operand::Rel(BranchTarget::Offset(0)))
            .assemble()
            .unwrap();
    }

    #[test]
    fn it_branches_to_nearby_label() {
        let mut asm = Assembler::new();
        asm.org(0x1000)
            .label("foo")
            .bcc(Operand::Rel(branch("bar")))
            .nop()
            .label("bar")
            .bcc(Operand::Rel(branch("foo")));
        println!("{}", asm);
        assert_eq!(asm.assemble().unwrap(), vec![0x90, 0x01, 0xEA, 0x90, 0xFB]);
    }

    #[test]
    #[should_panic(expected = "RelativeAddressOutOfRange(-131)")]
    fn it_cannot_branch_to_distant_label() {
        let mut asm = Assembler::new();
        asm.org(0x2000).label("start");
        for _ in 0..43 {
            asm.jmp(Operand::Abs(val(0x0000))); // 129 bytes of program
        }
        asm.bcc(Operand::Rel(branch("start")));
        asm.assemble().unwrap();
    }

    #[test]
    fn it_lists_with_out_of_range_label() {
        let mut asm = Assembler::new();
        asm.org(0x2000).label("start");
        for _ in 0..43 {
            asm.jmp(Operand::Abs(val(0x0000))); // 129 bytes of program
        }
        asm.bcc(Operand::Rel(branch("start")));
        println!("{}", asm.listing().unwrap());
    }
}

pub struct Assembler {
    pub org: u16,
    lines: Vec<Line>,
    next_label: Option<String>,
    opcode_map: isa::OpcodeByMnemonicAndAddressMode,
}

impl Assembler {
    pub fn new() -> Assembler {
        Self {
            org: 0x0000,
            lines: Vec::new(),
            next_label: None,
            opcode_map: isa::OpcodeByMnemonicAndAddressMode::build(),
        }
    }

    pub fn org(&mut self, addr: u16) -> &mut Assembler {
        self.org = addr;
        self
    }

    pub fn label(&mut self, l: &str) -> &mut Assembler {
        self.next_label = Some(l.to_string());
        self
    }

    // ----------------------------------------
    // Instructions

    pub fn adc(&mut self, op: Operand) -> &mut Assembler {
        self.push_instruction(Mnemonic::Adc, op)
    }

    pub fn and(&mut self, op: Operand) -> &mut Assembler {
        self.push_instruction(Mnemonic::And, op)
    }

    pub fn asl(&mut self, op: Operand) -> &mut Assembler {
        self.push_instruction(Mnemonic::Asl, op)
    }

    pub fn bcc(&mut self, op: Operand) -> &mut Assembler {
        self.push_instruction(Mnemonic::Bcc, op)
    }

    pub fn nop(&mut self) -> &mut Assembler {
        self.push_instruction(Mnemonic::Nop, Operand::Impl)
    }

    pub fn jmp(&mut self, op: Operand) -> &mut Self {
        self.push_instruction(Mnemonic::Jmp, op)
    }

    pub fn ldx(&mut self, op: Operand) -> &mut Self {
        self.push_instruction(Mnemonic::Ldx, op)
    }

    pub fn inx(&mut self) -> &mut Self {
        self.push_instruction(Mnemonic::Inx, Operand::Impl)
    }

    // ----------------------------------------

    pub fn assemble(&self) -> Result<Vec<u8>, Error> {
        let mut bin: Vec<u8> = Vec::new();

        let labtab = self.build_label_table();

        let mut addr = self.org;
        for line in self.lines.iter() {
            addr += 1 + (line.operand.length() as u16); // TODO: line.size()
            bin.push(line.instruction?.code);
            match self.op_value(addr, &line.operand, &labtab)? {
                OpValue::None => {}
                OpValue::U8(x) => bin.push(x),
                OpValue::U16(x) => {
                    bin.push(x as u8);
                    bin.push((x >> 8) as u8);
                }
            };
        }
        Ok(bin)
    }

    pub fn listing(&self) -> Result<String, fmt::Error> {
        use std::fmt::Write;
        let mut f = String::new();

        let labtab = self.build_label_table();

        writeln!(f, "* = ${:04X}", self.org)?;
        let mut addr = self.org;
        for line in self.lines.iter() {
            let base_addr = addr;
            addr += 1 + (line.operand.length() as u16); // TODO: line.size()
            let instruction = line.instruction?;
            let mut err: Option<Error> = None;
            let ophex = match self.op_value(addr, &line.operand, &labtab) {
                Ok(x) => match x {
                    OpValue::None => String::new(),
                    OpValue::U8(x) => format!("${:02X}", x),
                    OpValue::U16(x) => format!("${:02X} ${:02X}", x & 0xFF, (x >> 8)),
                },
                Err(e) => {
                    err = Some(e);
                    format!(" ??  ??")
                }
            };
            writeln!(
                f,
                "${:04X}  ${:02X} {:7}  {:16} {} {}{}{}",
                base_addr,
                instruction.code,
                ophex,
                if let Some(label) = &line.label {
                    format!("{}:", label)
                } else {
                    String::from("")
                },
                instruction.mnemonic,
                if let AddressMode::Immediate = instruction.mode {
                    "#"
                } else {
                    ""
                },
                line.operand,
                if let Some(e) = err {
                    format!(" ; {e:?}")
                } else {
                    String::from("")
                }
            )?;
        }
        Ok(f)
    }

    pub fn print_listing(&mut self) -> &mut Self {
        println!("{}", self.listing().unwrap());
        self
    }

    fn push_instruction(&mut self, mnemonic: Mnemonic, op: Operand) -> &mut Assembler {
        self.lines.push(Line {
            label: self.next_label.take(),
            instruction: self.opcode_map.get(mnemonic, op.mode()).map_err(Into::into),
            operand: op,
        });
        self
    }

    fn op_value(
        &self,
        addr: u16,
        op: &Operand,
        labtab: &HashMap<&str, u16>,
    ) -> Result<OpValue, Error> {
        use Operand::*;
        match op {
            A | Impl => Ok(OpValue::None),
            Abs(x) | AbsX(x) | AbsY(x) | Ind(x) => match x {
                Addr::Literal(x) => Ok(OpValue::U16(*x)),
                Addr::Label(x) => match labtab.get(x.as_str()) {
                    Some(addr) => Ok(OpValue::U16(*addr)),
                    None => Err(Error::LabelNotFound),
                },
            },
            Rel(x) => match x {
                BranchTarget::Offset(x) => Ok(OpValue::U8(*x as u8)),
                BranchTarget::Label(x) => {
                    let target_addr = match labtab.get(x.as_str()) {
                        Some(addr) => *addr,
                        None => return Err(Error::LabelNotFound),
                    };
                    let rel16: i16 = target_addr.wrapping_sub(addr) as i16;
                    let rel8: i8 = match rel16.try_into() {
                        Ok(x) => x,
                        Err(_) => {
                            return Err(Error::RelativeAddressOutOfRange(rel16));
                        }
                    };
                    Ok(OpValue::U8(rel8 as u8))
                }
            },
            Imm(x) | XInd(x) | IndY(x) | Z(x) | ZX(x) | ZY(x) => Ok(OpValue::U8(*x)),
        }
    }

    fn build_label_table(&self) -> HashMap<&str, u16> {
        let mut labtab: HashMap<&str, u16> = HashMap::new();
        let mut addr = self.org;
        for line in self.lines.iter() {
            if let Some(l) = &line.label {
                labtab.insert(l, addr);
            }
            addr += 1 + (line.operand.length() as u16); // TODO: line.size()
        }
        labtab
    }
}

impl fmt::Display for Assembler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.listing()?)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Operand::*;
        match self {
            A => write!(f, "A"),
            Abs(x) => write!(f, "{}", x),
            AbsX(x) => write!(f, "{},X", x),
            AbsY(x) => write!(f, "{},Y", x),
            Imm(x) => write!(f, "${:02X}", x),
            Impl => Ok(()),
            Ind(x) => write!(f, "({})", x),
            XInd(x) => write!(f, "(${:02X},X)", x),
            IndY(x) => write!(f, "(${:02X}),Y", x),
            Rel(x) => write!(f, "{}", x),
            Z(x) => write!(f, "${:02X}", x),
            ZX(x) => write!(f, "${:02X},X", x),
            ZY(x) => write!(f, "${:02X},Y", x),
        }
    }
}

impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Addr::Literal(a) => write!(f, "${:04X}", a),
            Addr::Label(l) => write!(f, "{}", l),
        }
    }
}

impl fmt::Display for BranchTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BranchTarget::Offset(x) => write!(f, "{}", x),
            BranchTarget::Label(l) => write!(f, "{}", l),
        }
    }
}

#[derive(Debug)]
struct Line {
    label: Option<String>,
    instruction: Result<Opcode, Error>,
    operand: Operand,
}

// Opcode Operands
#[derive(Debug)]
#[allow(dead_code)]
pub enum Operand {
    A,
    Abs(Addr),
    AbsX(Addr),
    AbsY(Addr),
    Imm(u8),
    Impl,
    Ind(Addr),
    XInd(u8),
    IndY(u8),
    Rel(BranchTarget),
    Z(u8),
    ZX(u8),
    ZY(u8),
}

impl Operand {
    fn mode(&self) -> AddressMode {
        match self {
            Operand::A => AddressMode::Accumulator,
            Operand::Abs(_) => AddressMode::Absolute,
            Operand::AbsX(_) => AddressMode::AbsoluteX,
            Operand::AbsY(_) => AddressMode::AbsoluteY,
            Operand::Imm(_) => AddressMode::Immediate,
            Operand::Impl => AddressMode::Implied,
            Operand::Ind(_) => AddressMode::Indirect,
            Operand::XInd(_) => AddressMode::XIndirect,
            Operand::IndY(_) => AddressMode::IndirectY,
            Operand::Rel(_) => AddressMode::Relative,
            Operand::Z(_) => AddressMode::Zeropage,
            Operand::ZX(_) => AddressMode::ZeropageX,
            Operand::ZY(_) => AddressMode::ZeropageY,
        }
    }

    fn length(&self) -> u16 {
        isa::operand_length(self.mode())
    }
}

#[derive(Debug)]
pub enum Addr {
    Literal(u16),
    Label(String),
}

// shorthand for a literal/numeric address, as opposed to a labelled address.
pub fn val(v: u16) -> Addr {
    Addr::Literal(v)
}

// shorthand for a labelled address, as opposed to a literal (numeric) address.
pub fn label(s: &str) -> Addr {
    Addr::Label(s.to_string())
}

#[derive(Debug)]
#[allow(unused)]
pub enum BranchTarget {
    Offset(i8),
    Label(String),
}

// shorthand for a nearby labelled branch target
pub fn branch(s: &str) -> BranchTarget {
    BranchTarget::Label(s.to_string())
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    IllegalAddressMode(Mnemonic, AddressMode),
    RelativeAddressOutOfRange(i16),
    LabelNotFound, // TODO: include label string
}

impl From<Error> for fmt::Error {
    fn from(_: Error) -> Self {
        Self
    }
}

impl From<isa::Error> for Error {
    fn from(err: isa::Error) -> Self {
        match err {
            isa::Error::IllegalAddressMode(m, am) => Error::IllegalAddressMode(m, am),
        }
    }
}

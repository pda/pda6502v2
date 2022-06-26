use std::collections::HashMap;
use std::fmt;

use crate::isa::instruction_list;
use crate::isa::AddressMode;
use crate::isa::Instruction;
use crate::isa::Mnemonic;

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
        assert_eq!(
            asm.nop()
                .nop()
                .jmp(Operand::Abs(Addr::Literal(0x1234)))
                .assemble()
                .unwrap(),
            vec![0xEA, 0xEA, 0x4C, 0x34, 0x12]
        );
    }

    #[test]
    fn it_assembles_labels() {
        let mut asm = Assembler::new();
        asm.nop()
            .label("foo")
            .jmp(Operand::Abs(Addr::Label(String::from("bar"))))
            .label("bar")
            .jmp(Operand::Abs(Addr::Label(String::from("foo"))));
        println!("{}", asm);
        assert_eq!(
            asm.assemble().unwrap(),
            vec![0x4C, 0x03, 0x00, 0x4C, 0x00, 0x00]
        );
    }

    #[test]
    #[should_panic(expected = "IllegalAddressMode(JMP, Relative)")]
    fn it_errors_on_illegal_address_mode() {
        let mut asm = Assembler::new();
        asm.jmp(Operand::Rel(0xFF)).assemble().unwrap();
    }
}

pub struct Assembler {
    pub org: u16,
    lines: Vec<Line>,
    next_label: Option<String>,
    opcode_table: HashMap<Mnemonic, HashMap<AddressMode, Instruction>>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Self {
            org: 0x0000,
            lines: Vec::new(),
            next_label: None,
            opcode_table: build_opcode_table(),
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

    pub fn nop(&mut self) -> &mut Assembler {
        self.push_instruction(Mnemonic::NOP, Operand::Impl)
    }

    pub fn jmp(&mut self, op: Operand) -> &mut Self {
        self.push_instruction(Mnemonic::JMP, op)
    }

    pub fn ldx(&mut self, op: Operand) -> &mut Self {
        self.push_instruction(Mnemonic::LDX, op)
    }

    pub fn inx(&mut self) -> &mut Self {
        self.push_instruction(Mnemonic::INX, Operand::Impl)
    }

    pub fn assemble(&self) -> Result<Vec<u8>, Error> {
        let mut bin: Vec<u8> = Vec::new();

        let labtab = self.build_label_table();

        for line in self.lines.iter() {
            bin.push(line.instruction?.code);
            match self.op_value(&line.operand, &labtab) {
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

    fn push_instruction(&mut self, mnemonic: Mnemonic, op: Operand) -> &mut Assembler {
        self.lines.push(Line {
            label: self.next_label.take(),
            instruction: self.find_instruction(&mnemonic, &op),
            operand: op,
        });
        self
    }

    fn find_instruction(&self, m: &Mnemonic, op: &Operand) -> Result<Instruction, Error> {
        let mode = op.mode();
        self.opcode_table
            .get(m)
            .unwrap() // all Mnemonic values should be in the HashMap
            .get(&mode) // might be None for this Operand's AddressMode
            .copied() // Option<&Instruction> -> Option<Instruction>
            .ok_or(Error::IllegalAddressMode(*m, mode))
    }

    fn op_value(&self, op: &Operand, labtab: &HashMap<&str, u16>) -> OpValue {
        match op {
            Operand::A => OpValue::None,
            Operand::Abs(x) => match x {
                Addr::Literal(x) => OpValue::U16(*x),
                Addr::Label(x) => OpValue::U16(*labtab.get(x.as_str()).unwrap()), // TODO: Result not unwrap
            },
            Operand::AbsX(x) => match x {
                Addr::Literal(x) => OpValue::U16(*x),
                Addr::Label(_) => todo!(),
            },
            Operand::AbsY(x) => match x {
                Addr::Literal(x) => OpValue::U16(*x),
                Addr::Label(_) => todo!(),
            },
            Operand::Imm(x) => OpValue::U8(*x),
            Operand::Impl => OpValue::None,
            Operand::Ind(x) => match x {
                Addr::Literal(x) => OpValue::U16(*x),
                Addr::Label(_) => todo!(),
            },
            Operand::XInd(x) => OpValue::U8(*x),
            Operand::IndY(x) => OpValue::U8(*x),
            Operand::Rel(x) => OpValue::U8(*x),
            Operand::Z(x) => OpValue::U8(*x),
            Operand::ZX(x) => OpValue::U8(*x),
            Operand::ZY(x) => OpValue::U8(*x),
        }
    }

    fn build_label_table(&self) -> HashMap<&str, u16> {
        let mut labtab: HashMap<&str, u16> = HashMap::new();
        let mut addr = self.org;
        for line in self.lines.iter() {
            if let Some(l) = &line.label {
                labtab.insert(l, addr);
            }
            addr += 1 + (line.operand.length() as u16);
        }
        labtab
    }
}

impl fmt::Display for Assembler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let labtab = self.build_label_table();

        writeln!(f, "* = ${:04X}", self.org)?;
        let mut addr = self.org;
        for line in self.lines.iter() {
            let instruction = line.instruction?;
            let opvalue = self.op_value(&line.operand, &labtab);
            let ophex = match opvalue {
                OpValue::None => format!(""),
                OpValue::U8(x) => format!("${:02X}", x),
                OpValue::U16(x) => format!("${:02X} ${:02X}", x & 0xFF, (x >> 8)),
            };
            writeln!(
                f,
                "${:04X}  ${:02X} {:7}  {:16} {:?} {}",
                addr,
                instruction.code,
                ophex,
                if let Some(label) = &line.label {
                    format!("{}:", label)
                } else {
                    String::from("")
                },
                instruction.mnemonic,
                line.operand,
            )?;
            addr += 1 + (line.operand.length() as u16);
        }
        Ok(())
    }
}

impl From<Error> for fmt::Error {
    fn from(_: Error) -> Self {
        Self
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::A => write!(f, "{}", "A"),
            Operand::Abs(x) => write!(f, "{}", x),
            Operand::AbsX(x) => write!(f, "{},X", x),
            Operand::AbsY(x) => write!(f, "{},Y", x),
            Operand::Imm(x) => write!(f, "${:02X}", x),
            Operand::Impl => Ok(()),
            Operand::Ind(x) => write!(f, "({})", x),
            Operand::XInd(x) => write!(f, "(${:02X},X)", x),
            Operand::IndY(x) => write!(f, "(${:02X}),Y", x),
            Operand::Rel(x) | Operand::Z(x) => write!(f, "${:02X}", x),
            Operand::ZX(x) => write!(f, "${:02X},X", x),
            Operand::ZY(x) => write!(f, "${:02X},Y", x),
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

#[derive(Debug, Clone, Copy)]
pub enum Error {
    IllegalAddressMode(Mnemonic, AddressMode),
}

#[derive(Debug)]
struct Line {
    label: Option<String>,
    instruction: Result<Instruction, Error>,
    operand: Operand,
}

// Instruction Operands
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
    Rel(u8),
    Z(u8),
    ZX(u8),
    ZY(u8),
}

impl Operand {
    fn mode(&self) -> AddressMode {
        use Operand::*;
        match self {
            A => AddressMode::Accumulator,
            Abs(_) => AddressMode::Absolute,
            AbsX(_) => AddressMode::AbsoluteX,
            AbsY(_) => AddressMode::AbsoluteY,
            Imm(_) => AddressMode::Immediate,
            Impl => AddressMode::Implied,
            Ind(_) => AddressMode::Indirect,
            XInd(_) => AddressMode::XIndirect,
            IndY(_) => AddressMode::IndirectY,
            Rel(_) => AddressMode::Relative,
            Z(_) => AddressMode::Zeropage,
            ZX(_) => AddressMode::ZeropageX,
            ZY(_) => AddressMode::ZeropageY,
        }
    }

    fn length(&self) -> u8 {
        use Operand::*;
        match self {
            A | Impl => 0,
            Imm(_) | XInd(_) | IndY(_) | Rel(_) | Z(_) | ZX(_) | ZY(_) => 1,
            Abs(_) | AbsX(_) | AbsY(_) | Ind(_) => 2,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum Addr {
    Literal(u16),
    Label(String),
}

enum OpValue {
    None,
    U8(u8),
    U16(u16),
}

fn build_opcode_table() -> HashMap<Mnemonic, HashMap<AddressMode, Instruction>> {
    let mut map: HashMap<_, HashMap<_, _>> = HashMap::new();
    for instruction in instruction_list() {
        map.entry(instruction.mnemonic)
            .or_default()
            .insert(instruction.mode, instruction);
    }
    map
}

pub fn val(v: u16) -> Addr {
    Addr::Literal(v)
}

pub fn label(s: &str) -> Addr {
    Addr::Label(s.to_string())
}

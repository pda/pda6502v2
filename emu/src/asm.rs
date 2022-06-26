use std::collections::HashMap;

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
            asm.nop().nop().jmp(Op::Abs(0x1234)).assemble().unwrap(),
            vec![0xEA, 0xEA, 0x4C, 0x34, 0x12]
        );
    }

    #[test]
    #[should_panic(expected = "IllegalAddressMode(JMP, Relative)")]
    fn it_errors_on_illegal_address_mode() {
        let mut asm = Assembler::new();
        asm.jmp(Op::Rel(0xFF)).assemble().unwrap();
    }
}

pub struct Assembler {
    lines: Vec<Line>,
    // next_label: Option<String>,
    opcode_table: HashMap<Mnemonic, HashMap<AddressMode, Instruction>>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Self {
            lines: Vec::new(),
            // next_label: None,
            opcode_table: build_opcode_table(),
        }
    }

    pub fn nop(&mut self) -> &mut Assembler {
        self.push_instruction(Mnemonic::NOP, Op::Impl)
    }

    pub fn jmp(&mut self, op: Op) -> &mut Self {
        self.push_instruction(Mnemonic::JMP, op)
    }

    pub fn inx(&mut self) -> &mut Self {
        self.push_instruction(Mnemonic::INX, Op::Impl)
    }

    pub fn assemble(&mut self) -> Result<Vec<u8>, Error> {
        let mut bin: Vec<u8> = Vec::new();
        for line in self.lines.iter() {
            bin.push(line.instruction?.code);
            match op_value(&line.operand) {
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

    fn push_instruction(&mut self, mnemonic: Mnemonic, op: Op) -> &mut Assembler {
        self.lines.push(Line {
            // label: self.next_label.take(),
            instruction: self.find_instruction(&mnemonic, &op),
            operand: op,
        });
        self
    }

    fn find_instruction(&self, m: &Mnemonic, op: &Op) -> Result<Instruction, Error> {
        let mode = address_mode_for_op(op);
        self.opcode_table
            .get(m)
            .unwrap() // all Mnemonic values should be in the HashMap
            .get(&mode) // might be None for this Op's AddressMode
            .copied() // Option<&Instruction> -> Option<Instruction>
            .ok_or(Error::IllegalAddressMode(*m, mode))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    IllegalAddressMode(Mnemonic, AddressMode),
}

struct Line {
    // label: Option<String>,
    instruction: Result<Instruction, Error>,
    operand: Op,
}

// Instruction Operands
#[derive(Debug)]
#[allow(dead_code)]
pub enum Op {
    A,
    Abs(u16),
    AbsX(u16),
    AbsY(u16),
    Imm(u8), // immediate
    Impl,
    Ind(u16),
    XInd(u8),
    IndY(u8),
    Rel(u8),
    Z(u8),
    ZX(u8),
    ZY(u8),
}

fn address_mode_for_op(op: &Op) -> AddressMode {
    match op {
        Op::A => AddressMode::Accumulator,
        Op::Abs(_) => AddressMode::Absolute,
        Op::AbsX(_) => AddressMode::AbsoluteX,
        Op::AbsY(_) => AddressMode::AbsoluteY,
        Op::Imm(_) => AddressMode::Immediate,
        Op::Impl => AddressMode::Implied,
        Op::Ind(_) => AddressMode::Indirect,
        Op::XInd(_) => AddressMode::XIndirect,
        Op::IndY(_) => AddressMode::IndirectY,
        Op::Rel(_) => AddressMode::Relative,
        Op::Z(_) => AddressMode::Zeropage,
        Op::ZX(_) => AddressMode::ZeropageX,
        Op::ZY(_) => AddressMode::ZeropageY,
    }
}

enum OpValue {
    None,
    U8(u8),
    U16(u16),
}

fn op_value(op: &Op) -> OpValue {
    match op {
        Op::A => OpValue::None,
        Op::Abs(x) => OpValue::U16(*x),
        Op::AbsX(x) => OpValue::U16(*x),
        Op::AbsY(x) => OpValue::U16(*x),
        Op::Imm(x) => OpValue::U8(*x),
        Op::Impl => OpValue::None,
        Op::Ind(x) => OpValue::U16(*x),
        Op::XInd(x) => OpValue::U8(*x),
        Op::IndY(x) => OpValue::U8(*x),
        Op::Rel(x) => OpValue::U8(*x),
        Op::Z(x) => OpValue::U8(*x),
        Op::ZX(x) => OpValue::U8(*x),
        Op::ZY(x) => OpValue::U8(*x),
    }
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

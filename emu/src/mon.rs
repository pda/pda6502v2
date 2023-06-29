use crate::asm;
use crate::isa;

use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::dec::Decoder;

pub struct Monitor {
    decoder: Decoder,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            decoder: Decoder::new(),
        }
    }

    pub fn step(&self, bus: &mut Bus, cpu: &Cpu) {
        println!("{cpu:?}");
        let code = bus.read(cpu.pc);
        let opcode = self.decoder.opcode(code);

        match opcode {
            None => println!("  illegal opcode: {code:02X}"),
            Some(opcode) => println!("  {}", describe_opcode(&opcode, cpu.pc, &bus)),
        }
    }
}

fn describe_opcode(opcode: &isa::Opcode, pc: u16, bus: &Bus) -> String {
    use std::collections::HashMap;

    let addr = pc + 1; // operand address; one byte after the opcode
    let operand = match opcode.mode {
        isa::AddressMode::Absolute => asm::Operand::Abs(asm::Addr::Literal(bus.read_u16(addr))), // $LLHH
        isa::AddressMode::AbsoluteX => asm::Operand::AbsX(asm::Addr::Literal(bus.read_u16(addr))), // $LLHH,X
        isa::AddressMode::AbsoluteY => asm::Operand::AbsY(asm::Addr::Literal(bus.read_u16(addr))), // $LLHH,Y
        isa::AddressMode::Accumulator => asm::Operand::A,
        isa::AddressMode::Immediate => asm::Operand::Imm(bus.read(addr)), // $BB
        isa::AddressMode::Implied => asm::Operand::Impl,
        isa::AddressMode::Indirect => asm::Operand::Ind(asm::Addr::Literal(bus.read_u16(addr))), // ($LLHH)
        isa::AddressMode::IndirectY => asm::Operand::IndY(bus.read(addr)), // ($LL),Y
        isa::AddressMode::Relative => {
            asm::Operand::Rel(asm::BranchTarget::Offset(bus.read(addr) as i8))
        } // $BB (signed)
        isa::AddressMode::XIndirect => asm::Operand::XInd(bus.read(addr)), // ($LL,X)
        isa::AddressMode::Zeropage => asm::Operand::Z(bus.read(addr)),     // $LL
        isa::AddressMode::ZeropageX => asm::Operand::ZX(bus.read(addr)),   // $LL,X
        isa::AddressMode::ZeropageY => asm::Operand::ZY(bus.read(addr)),   // $LL,Y
    };

    let line = asm::Line::Instruction(asm::InstructionLine {
        label: None,
        instruction: Ok(*opcode),
        operand,
    });

    let mut buf = String::new();
    line.fmt(&mut buf, pc, addr, &HashMap::new()).unwrap();

    buf
}

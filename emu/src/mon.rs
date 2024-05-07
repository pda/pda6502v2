use crate::asm;
use crate::cpu;
use crate::dbginfo;
use crate::isa;

use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::dec::Decoder;

pub struct Monitor {
    decoder: Decoder,

    #[allow(unused)]
    dbginfo: dbginfo::Info,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            decoder: Decoder::new(),
            dbginfo: dbginfo::load("../os/debug.out").unwrap(),
        }
    }

    pub fn reset(&self, bus: &mut Bus) {
        let addr = bus.read_u16(cpu::VEC_RES);
        let label = self.dbginfo.label(addr).unwrap_or("");
        println!(
            "RESET: VEC_RES {:#06X} -> {} {:#06X}",
            cpu::VEC_RES,
            label,
            addr
        );
    }

    pub fn step(&self, bus: &mut Bus, cpu: &Cpu) {
        print!("[{cpu:?}] ");
        let code = bus.read(cpu.pc);
        let opcode = self.decoder.opcode(code);

        match opcode {
            None => println!("  illegal opcode: {code:02X}"),
            Some(opcode) => print!("{}", self.describe_opcode(&opcode, cpu, &bus)),
        }
    }

    fn label(&self, addr: u16, lpad: &str, rpad: &str) -> String {
        self.dbginfo
            .label(addr)
            .map(|x| lpad.to_owned() + x + rpad)
            .unwrap_or("".to_string())
    }

    fn describe_opcode(&self, opcode: &isa::Opcode, cpu: &Cpu, bus: &Bus) -> String {
        use std::collections::HashMap;

        let addr = cpu.pc + 1; // operand address; one byte after the opcode
        let operand = match opcode.mode {
            isa::AddressMode::Absolute => asm::Operand::Abs(asm::Addr::Literal(bus.read_u16(addr))), // $LLHH
            isa::AddressMode::AbsoluteX => {
                asm::Operand::AbsX(asm::Addr::Literal(bus.read_u16(addr)))
            } // $LLHH,X
            isa::AddressMode::AbsoluteY => {
                asm::Operand::AbsY(asm::Addr::Literal(bus.read_u16(addr)))
            } // $LLHH,Y
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

        let comment: Option<String> = match operand {
            asm::Operand::A => Some(format!("A:#${0:02X}:{0:#010b}", cpu.a)),
            asm::Operand::Abs(ref addr) => match addr {
                asm::Addr::Literal(val) => Some(format!(
                    "→ {}#${:02X}",
                    self.label(*val, "", " "),
                    bus.read(*val)
                )),
                asm::Addr::Label(_text) => todo!(),
            },
            asm::Operand::AbsX(ref addr) => match addr {
                asm::Addr::Literal(val) => {
                    let indexed = val.wrapping_add(cpu.x as u16);
                    Some(format!("→ ${:04X} -> #${:02X}", indexed, bus.read(indexed)))
                }
                asm::Addr::Label(_text) => todo!(),
            },
            asm::Operand::AbsY(ref addr) => match addr {
                asm::Addr::Literal(val) => {
                    let indexed = val.wrapping_add(cpu.y as u16);
                    Some(format!("→ ${:04X} -> #${:02X}", indexed, bus.read(indexed)))
                }
                asm::Addr::Label(_text) => todo!(),
            },
            asm::Operand::Imm(val) => Some(format!("→ #${val:02X}")),
            asm::Operand::Impl => None,
            asm::Operand::Ind(ref addr) => match addr {
                asm::Addr::Literal(val) => {
                    let indirect = bus.read_u16(*val);
                    Some(format!(
                        "→ ${:04X} -> #${:02X}",
                        indirect,
                        bus.read(indirect)
                    ))
                }
                asm::Addr::Label(_text) => todo!(),
            },
            asm::Operand::XInd(zp) => {
                let indirect = zp.wrapping_add(cpu.x) as u16;
                Some(format!(
                    "→ ${:02X} → #${:02X}",
                    indirect,
                    bus.read(indirect)
                ))
            }
            asm::Operand::IndY(zp) => {
                let indirect = bus.read_u16(zp as u16);
                let indexed = indirect.wrapping_add(cpu.y as u16);
                Some(format!(
                    "→ ${:04X},Y → ${:04X} → #${:02X}",
                    indirect,
                    indexed,
                    bus.read(indexed)
                ))
            }
            asm::Operand::Rel(ref target) => match target {
                asm::BranchTarget::Offset(offset) => Some(format!(
                    "→ ${:04X}",
                    cpu.pc.wrapping_add_signed(*offset as i16) as u16
                )),
                asm::BranchTarget::Label(_text) => todo!(),
            },
            asm::Operand::Z(zp) => Some(format!("→ #${:02X}", bus.read(zp as u16))),
            asm::Operand::ZX(zp) => {
                let indexed = zp.wrapping_add(cpu.x);
                Some(format!(
                    "→ ${:02X} → #${:02X}",
                    indexed,
                    bus.read(indexed as u16)
                ))
            }
            asm::Operand::ZY(zp) => {
                let indexed = zp.wrapping_add(cpu.y);
                Some(format!(
                    "→ ${:02X} → #${:02X}",
                    indexed,
                    bus.read(indexed as u16)
                ))
            }
        };

        let line = asm::Line::Instruction(asm::InstructionLine {
            label: self.dbginfo.label(cpu.pc).map(|x| x.to_string()),
            instruction: Ok(*opcode),
            operand,
            comment,
        });

        let mut buf = String::new();
        line.fmt(&mut buf, cpu.pc, addr, &HashMap::new()).unwrap();

        buf
    }
}

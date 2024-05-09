use lazy_static::lazy_static;
use regex::Regex;

use crate::asm;
use crate::cpu;
use crate::dbginfo;
use crate::isa;

use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::dec::Decoder;

lazy_static! {
    static ref STAT_INACTIVE_RE: Regex = Regex::new(r"[NVBDIZC]").unwrap();
    static ref LINE_RE: Regex = Regex::new(
        r"(?x)
        (?<addr>[0-9A-Z]{4})
        \s\|\s
        (?<bytecode>(?:[0-9A-Z\s]{2}\s?){3})
        \|\s
        (?<label>\S+:)?
        (?<labelpad>\s+)
        (?<mnemonic>[A-Z]{3})?
        (?<operand>\s\S+)?
        (?<comment>\s;.*)?
        "
    )
    .unwrap();
}

pub struct Monitor {
    decoder: Decoder,

    prev_reg: Reg,

    #[allow(unused)]
    dbginfo: dbginfo::Info,
}

#[derive(Default)]
struct Reg {
    s: u8,
    a: u8,
    x: u8,
    y: u8,
}

impl Reg {
    fn update(&mut self, cpu: &Cpu) {
        self.s = cpu.s;
        self.a = cpu.a;
        self.x = cpu.x;
        self.y = cpu.y;
    }
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            decoder: Decoder::new(),
            prev_reg: Reg::default(),
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

    pub fn step(&mut self, bus: &mut Bus, cpu: &Cpu) {
        print!(
            "\x1b[2mPC:{:04X} S:{} A:{} X:{} Y:{} P:{}\x1b[0m  ",
            cpu.pc,
            diff(cpu.s, self.prev_reg.s, "22;32", "2;39"),
            diff(cpu.a, self.prev_reg.a, "22;32", "2;39"),
            diff(cpu.x, self.prev_reg.x, "22;32", "2;39"),
            diff(cpu.y, self.prev_reg.y, "22;32", "2;39"),
            STAT_INACTIVE_RE.replace_all(&cpu::stat(&cpu.p), "\x1b[22;94m${0}\x1b[2;39m")
        );
        self.prev_reg.update(cpu);

        let code = bus.read(cpu.pc);
        let opcode = self.decoder.opcode(code);

        match opcode {
            None => println!("  illegal opcode: {code:02X}"),
            Some(opcode) => print!(
                "{}",
                LINE_RE.replace(
                    &self.describe_opcode(&opcode, cpu, &bus),
                    "${addr} \x1b[2m${bytecode} \x1b[22;33m${label}\x1b[39m${labelpad}${mnemonic}${operand}\x1b[2m${comment}\x1b[22m"
                )
            ),
        }
    }

    fn label(&self, addr: u16) -> String {
        self.dbginfo
            .label(addr)
            .map(|x| "\x1b[22;33m".to_owned() + x + "\x1b[2;39m ")
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
                asm::Addr::Literal(val) => {
                    Some(format!("→ {}{}", self.label(*val), bus.name_for_read(*val)))
                }
                asm::Addr::Label(_text) => todo!(),
            },
            asm::Operand::AbsX(ref addr) => match addr {
                asm::Addr::Literal(val) => {
                    let indexed = val.wrapping_add(cpu.x as u16);
                    Some(format!(
                        "→ ${:04X} -> {}{}",
                        indexed,
                        self.label(indexed),
                        bus.name_for_read(indexed)
                    ))
                }
                asm::Addr::Label(_text) => todo!(),
            },
            asm::Operand::AbsY(ref addr) => match addr {
                asm::Addr::Literal(val) => {
                    let indexed = val.wrapping_add(cpu.y as u16);
                    Some(format!(
                        "→ ${:04X} -> {}{}",
                        indexed,
                        self.label(indexed),
                        bus.name_for_read(indexed)
                    ))
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

fn diff(a: u8, b: u8, style: &str, reset: &str) -> String {
    if a == b {
        format!("{a:02X}")
    } else {
        format!("\x1b[{style}m{a:02X}\x1b[{reset}m")
    }
}

use crate::bus::Bus;
use crate::isa::instruction_list;
use crate::isa::AddressMode;
use crate::isa::Instruction;
use crate::isa::Mnemonic;

use core::fmt::Debug;
use core::fmt::Formatter;

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
        use AddressMode::*;
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
            Mnemonic::INX => match instruction.mode {
                Implied => {
                    self.x = self.x.wrapping_add(1);
                    // TODO: update self.status N and Z bits
                }
                other => panic!("illegal AddressMode: {:?}", other),
            },
            // Mnemonic::INY => {}
            Mnemonic::JMP => match instruction.mode {
                Absolute => self.pc = self.bus.read16(self.pc),
                Indirect => todo!("{:?}", instruction.mode),
                other => panic!("illegal AddressMode: {:?}", other),
            },
            // Mnemonic::JSR => {}
            // Mnemonic::LDA => {}
            Mnemonic::LDX => match instruction.mode {
                Immediate => {
                    self.x = self.bus.read(self.pc);
                    self.pc += 1;
                    // TODO: update self.status N and Z bits
                }
                Zeropage | ZeropageY | Absolute | AbsoluteY => todo!("{:?}", instruction.mode),
                other => panic!("illegal AddressMode: {:?}", other),
            },
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
            other => todo!("{:?}", other),
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

pub fn build_opcode_table() -> [Option<Instruction>; 256] {
    let mut optab = [None; 256];
    for opcode in instruction_list() {
        let code = opcode.code as usize;
        optab[code] = Some(opcode);
    }
    return optab;
}

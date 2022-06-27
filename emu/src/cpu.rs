use std::fmt;

use crate::bus;
use crate::isa;

// A 65C02-like CPU
pub struct Cpu {
    bus: bus::Bus,
    pc: u16, // program counter
    sp: u8,  // stack pointer
    a: u8,   // accumulator
    x: u8,   // X register
    y: u8,   // Y register
    sr: u8,  // status register

    optab: [Option<isa::Opcode>; 256],
}

impl Cpu {
    pub fn new(bus: bus::Bus) -> Cpu {
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

    // Reset internal CPU state, as if the reset line had been asserted.
    pub fn reset(&mut self) {
        self.pc = read16(&self.bus, 0xFFFC);
        self.sp = 0x00;
        self.a = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        self.sr = 0x00;
    }

    // Load and execute a single instruction.
    pub fn step(&mut self) {
        let opcode = self.bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        match self.optab[opcode as usize] {
            None => panic!("illegal opcode"),
            Some(instruction) => self.execute(instruction),
        }
    }

    // Execute an instruction, reading the operands for its address mode from the bus.
    fn execute(&mut self, instruction: isa::Opcode) {
        use isa::AddressMode::*;
        use isa::Mnemonic as M;

        println!("{:?}", instruction);
        match instruction.mnemonic {
            // M::ADC => {}
            // M::AND => {}
            // M::ASL => {}
            // M::BCC => {}
            // M::BCS => {}
            // M::BEQ => {}
            // M::BIT => {}
            // M::BMI => {}
            // M::BNE => {}
            // M::BPL => {}
            // M::BRK => {}
            // M::BVC => {}
            // M::BVS => {}
            // M::CLC => {}
            // M::CLD => {}
            // M::CLI => {}
            // M::CLV => {}
            // M::CMP => {}
            // M::CPX => {}
            // M::CPY => {}
            // M::DEC => {}
            // M::DEX => {}
            // M::DEY => {}
            // M::EOR => {}
            // M::INC => {}
            M::INX => match instruction.mode {
                Implied => {
                    self.x = self.x.wrapping_add(1);
                    // TODO: update self.status N and Z bits
                }
                other => panic!("illegal AddressMode: {:?}", other),
            },
            // M::INY => {}
            M::JMP => match instruction.mode {
                Absolute => self.pc = read16(&self.bus, self.pc),
                Indirect => todo!("{:?}", instruction.mode),
                other => panic!("illegal AddressMode: {:?}", other),
            },
            // M::JSR => {}
            // M::LDA => {}
            M::LDX => match instruction.mode {
                Immediate => {
                    self.x = self.bus.read(self.pc);
                    self.pc += 1;
                    // TODO: update self.status N and Z bits
                }
                Zeropage | ZeropageY | Absolute | AbsoluteY => todo!("{:?}", instruction.mode),
                other => panic!("illegal AddressMode: {:?}", other),
            },
            // M::LDY => {}
            // M::LSR => {}
            M::NOP => {}
            // M::ORA => {}
            // M::PHA => {}
            // M::PHP => {}
            // M::PLA => {}
            // M::PLP => {}
            // M::ROL => {}
            // M::ROR => {}
            // M::RTI => {}
            // M::RTS => {}
            // M::SBC => {}
            // M::SEC => {}
            // M::SED => {}
            // M::SEI => {}
            // M::STA => {}
            // M::STX => {}
            // M::STY => {}
            // M::TAX => {}
            // M::TAY => {}
            // M::TSX => {}
            // M::TXA => {}
            // M::TXS => {}
            // M::TYA => {}
            other => todo!("{:?}", other),
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let stat = stat(&self.sr);
        f.write_fmt(format_args!(
            "Cpu {{ SR: {} PC: ${:04X} SP: ${:02X} A: ${:02X} X: ${:02X} Y: ${:02X} }}",
            stat, self.pc, self.sp, self.a, self.x, self.y,
        ))
    }
}

// read a u16 in little-endian order from the bus
fn read16(bus: &bus::Bus, addr: u16) -> u16 {
    let lo = bus.read(addr as u16);
    let hi = bus.read(addr.wrapping_add(1) as u16);
    ((hi as u16) << 8) | (lo as u16)
}

// a string representation of the 8-bit status register;
// enabled bits are upper case, disabled bits are lower case.
fn stat(sr: &u8) -> String {
    "nv-bdizc"
        .chars()
        .enumerate()
        .map(|(i, x)| {
            if sr >> (7 - i) & 1 == 1 {
                x.to_ascii_uppercase()
            } else {
                x
            }
        })
        .collect()
}

// Build an array of isa::Opcode indexed by by their u8 opcode.
pub fn build_opcode_table() -> [Option<isa::Opcode>; 256] {
    let mut optab = [None; 256];
    for opcode in isa::opcode_list() {
        optab[opcode.code as usize] = Some(opcode);
    }
    optab
}

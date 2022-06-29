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
            // M::Adc => {}
            // M::And => {}
            // M::Asl => {}
            // M::Bcc => {}
            // M::Bcs => {}
            // M::Beq => {}
            // M::Bit => {}
            // M::Bmi => {}
            // M::Bne => {}
            // M::Bpl => {}
            // M::Brk => {}
            // M::Bvc => {}
            // M::Bvs => {}
            // M::Clc => {}
            // M::Cld => {}
            // M::Cli => {}
            // M::Clv => {}
            // M::Cmp => {}
            // M::Cpx => {}
            // M::Cpy => {}
            // M::Dec => {}
            // M::Dex => {}
            // M::Dey => {}
            // M::Eor => {}
            // M::Inc => {}
            M::Inx => match instruction.mode {
                Implied => {
                    self.x = self.x.wrapping_add(1);
                    // TODO: update self.status N and Z bits
                }
                other => panic!("illegal AddressMode: {:?}", other),
            },
            // M::Iny => {}
            M::Jmp => match instruction.mode {
                Absolute => self.pc = read16(&self.bus, self.pc),
                Indirect => todo!("{:?}", instruction.mode),
                other => panic!("illegal AddressMode: {:?}", other),
            },
            // M::Jsr => {}
            // M::Lda => {}
            M::Ldx => match instruction.mode {
                Immediate => {
                    self.x = self.bus.read(self.pc);
                    self.pc += 1;
                    // TODO: update self.status N and Z bits
                }
                Zeropage | ZeropageY | Absolute | AbsoluteY => todo!("{:?}", instruction.mode),
                other => panic!("illegal AddressMode: {:?}", other),
            },
            // M::Ldy => {}
            // M::Lsr => {}
            M::Nop => {}
            // M::Ora => {}
            // M::Pha => {}
            // M::Php => {}
            // M::Pla => {}
            // M::Plp => {}
            // M::Rol => {}
            // M::Ror => {}
            // M::Rti => {}
            // M::Rts => {}
            // M::Sbc => {}
            // M::Sec => {}
            // M::Sed => {}
            // M::Sei => {}
            // M::Sta => {}
            // M::Stx => {}
            // M::Sty => {}
            // M::Tax => {}
            // M::Tay => {}
            // M::Tsx => {}
            // M::Txa => {}
            // M::Txs => {}
            // M::Tya => {}
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

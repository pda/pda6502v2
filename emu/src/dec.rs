use crate::isa;

pub struct Decoder {
    table: [Option<isa::Opcode>; 256],
}

impl Decoder {
    pub fn new() -> Self {
        Decoder {
            table: build_opcode_table(),
        }
    }

    pub fn opcode(&self, code: u8) -> Option<isa::Opcode> {
        self.table[code as usize]
    }
}

// Build an array of isa::Opcode indexed by by their u8 opcode.
fn build_opcode_table() -> [Option<isa::Opcode>; 256] {
    let mut optab = [None; 256];
    for opcode in isa::opcode_list() {
        optab[opcode.code as usize] = Some(opcode);
    }
    optab
}

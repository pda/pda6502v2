use std::fmt;

use pda6502v2emu::bus::Bus;
use pda6502v2emu::cpu;
use pda6502v2emu::cpu::Cpu;
use pda6502v2emu::isa;

use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct ProcessorTestCase {
    name: String,
    initial: ProcessorState,
    #[serde(rename = "final")]
    after: ProcessorState,
    cycles: Vec<(u16, u8, String)>,
}

#[allow(unused)]
#[derive(Deserialize)]
struct ProcessorState {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    ram: Vec<(u16, u8)>,
}

impl fmt::Debug for ProcessorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let stat = cpu::stat(&self.p);
        f.write_str("ProcessorState { ")?;
        f.write_fmt(format_args!(
            "P:{} PC:${:04X} S:${:02X} A:${:02X} X:${:02X} Y:${:02X} RAM:{{",
            stat, self.pc, self.s, self.a, self.x, self.y,
        ))?;
        for (addr, val) in self.ram.as_slice() {
            f.write_fmt(format_args!("${addr:04X}:${val:02X} "))?;
        }
        // TODO: Debug RAM
        f.write_str("} }")
    }
}

fn init_cpu(state: &ProcessorState) -> Cpu {
    let mut cpu = Cpu::new(Bus::default());
    cpu.pc = state.pc;
    cpu.p = state.p;
    cpu.a = state.a;
    cpu.x = state.x;
    cpu.y = state.y;
    cpu.s = state.s;
    for (addr, value) in state.ram.as_slice() {
        // println!("{addr:#06X} ← {value:#04X}");
        cpu.bus.write(*addr, *value);
    }
    cpu
}

macro_rules! assert_eq_hex {
    ($a:expr, $b:expr) => {
        assert_eq!(
            $a,
            $b,
            "{}:{}:{:#04X}:{:#010b} != {}:{:#04X}:{:#010b}",
            stringify!($a),
            $a,
            $a,
            $a,
            $b,
            $b,
            $b,
        );
    };
}

macro_rules! assert_eq_p {
    ($a:expr, $b:expr) => {
        assert_eq!(
            $a,
            $b,
            "{}:{}:{:#04X}:{} != {}:{:#04X}:{}",
            stringify!($a),
            $a,
            $a,
            cpu::stat(&$a),
            $b,
            $b,
            cpu::stat(&$b),
        );
    };
}

macro_rules! assert_mem {
    ($cpu:expr, $addr:expr, $expected:expr) => {
        let actual = $cpu.bus.read($addr);
        assert_eq!(
            actual, $expected,
            "addr:{}:{:#06X} actual:{}:{:#04X}:{:#010b} != expected:{}:{:#04X}:{:#010b}",
            $addr, $addr, actual, actual, actual, $expected, $expected, $expected,
        );
    };
}

fn assert_cpu(cpu: &Cpu, expected: &ProcessorState) {
    assert_eq_hex!(cpu.pc, expected.pc);
    assert_eq_p!(cpu.p, expected.p);
    assert_eq_hex!(cpu.a, expected.a);
    assert_eq_hex!(cpu.x, expected.x);
    assert_eq_hex!(cpu.y, expected.y);
    assert_eq_hex!(cpu.s, expected.s);
    for (addr, expected) in expected.ram.as_slice() {
        assert_mem!(cpu, *addr, *expected);
    }
}

#[test]
fn test_bulk() {
    use std::fs;

    for opcode in isa::opcode_list() {
        if opcode.code == 0x6D {
            continue;
        }
        let file_path =
            format!("../../ProcessorTests/wdc65c02/v1/{:02X}.json", opcode.code).to_owned();
        let data = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
        let tests: Vec<ProcessorTestCase> = serde_json::from_str(&data).unwrap();

        for t in tests {
            println!("-------- {}", t.name);
            println!("initial: {:?}", t.initial);
            println!("after:   {:?}", t.after);
            let mut cpu = init_cpu(&t.initial);
            println!("before: {cpu:?}");
            cpu.step();
            println!("after:  {cpu:?}");
            assert_cpu(&cpu, &t.after);
        }
    }
}

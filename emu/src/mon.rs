use crate::bus::Bus;
use crate::cpu::Cpu;

pub struct Monitor {}

impl Monitor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn step(&self, bus: &mut Bus, cpu: &Cpu) {
        println!("Monitor: {cpu:?}");
        let opcode = cpu.fetch(bus);
        println!("  next instruction: {opcode:?}");
        // TODO: refactor CPU to peek at operands
    }
}

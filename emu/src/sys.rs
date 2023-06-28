use crate::bus::Bus;
use crate::cpu::Cpu;

pub struct Sys {
    pub bus: Bus,
    cpu: Cpu,
}

impl Sys {
    pub fn new() -> Self {
        Self {
            bus: Bus::default(),
            cpu: Cpu::new(),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&self.bus)
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.bus)
    }
}

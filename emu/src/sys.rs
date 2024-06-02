use std::fs;

use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::mon::Monitor;

pub struct Sys {
    pub bus: Bus,
    cpu: Cpu,
    monitor: Monitor,
}

impl Sys {
    pub fn new() -> Self {
        Self {
            bus: Bus::new(),
            cpu: Cpu::new(),
            monitor: Monitor::new(),
        }
    }

    pub fn reset(&mut self) {
        self.bus.reset();
        self.bus.load(0xF000, fs::read("../os/os.rom").unwrap());
        self.monitor.reset(&mut self.bus);
        self.cpu.reset(&mut self.bus)
    }

    pub fn step(&mut self) {
        self.bus.step();
        if self.bus.is_interrupt() {
            self.cpu.interrupt(&mut self.bus);
        }
        self.monitor.step(&mut self.bus, &self.cpu);
        self.cpu.step(&mut self.bus);
    }
}

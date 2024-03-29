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
            bus: Bus::default(),
            cpu: Cpu::new(),
            monitor: Monitor::new(),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&self.bus)
    }

    pub fn step(&mut self) {
        self.monitor.step(&mut self.bus, &self.cpu);
        self.cpu.step(&mut self.bus);
    }
}

mod bus;
mod cpu;

use crate::bus::Bus;
use crate::cpu::Cpu;

fn main() {
    let mut bus = Bus::default();

    // fill with NOPs for now
    for addr in 0..=65535 {
        bus.write(addr, cpu::OP_NOP)
    }
    // write a rando reset vector
    bus.write(0xFFFC, 0x34);
    bus.write(0xFFFD, 0x12);

    let mut cpu = Cpu::new(bus);
    cpu.reset();

    for _ in 0..10 {
        println!("{:?}", cpu);
        cpu.step();
    }
}

mod bios;
mod bus;
mod consts;
mod cpu;
mod map;

use crate::bios::Bios;
use crate::bus::Bus;
use crate::cpu::CPU;

fn main() {
    let bios = Bios::new("bios/SCPH1001.BIN");

    let bus = Bus::new(bios);

    let mut cpu = CPU::new(bus);

    cpu.run_next_instruction();
}

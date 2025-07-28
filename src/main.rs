mod consts;
mod memory;

use crate::memory::bios::Bios;
use crate::memory::bus::Bus;
use crate::memory::cpu::CPU;

fn main() {
    let bios = Bios::new("bios/SCPH1001.BIN");

    let bus = Bus::new(bios);

    let mut cpu = CPU::new(bus);

    cpu.run_next_instruction();
}

mod processor;
mod bus;
mod bios;

use processor::Processor;
use bus::Bus;
use bios::Bios;

fn main() {
    let bios = Bios::new("bios/SCPH1001.BIN");

    let bus = Bus::new(bios);

    let mut cpu = CPU::new(bus);

    cpu.run_next_instruction();

       
}


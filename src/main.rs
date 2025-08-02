mod consts;
mod libs;

use crate::libs::bios::Bios;
use crate::libs::bus::Bus;
use crate::libs::cpu::CPU;

fn main() {
    println!("{:032b}", 0x408c6000);

    let bios = Bios::new("bios/SCPH1001.BIN");

    let bus = Bus::new(bios);

    let mut cpu = CPU::new(bus);

    loop {
        cpu.run_next_opcode();
    }
}

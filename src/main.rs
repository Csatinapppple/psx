mod consts;
mod libs;

use crate::libs::bios::Bios;
use crate::libs::bus::Bus;
use crate::libs::cpu::CPU;
use crate::libs::ram::Ram;

fn main() {
    println!("{:032b}", 0x1420fffc);
    
    let bios = Bios::new("bios/SCPH1001.BIN");
    let ram = Ram::new();

    let bus = Bus::new(bios, ram);

    let mut cpu = CPU::new(bus);

    loop {
        cpu.run_next_opcode();
    }
}

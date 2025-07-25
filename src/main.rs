mod bios;
mod bus;
mod consts;
mod map;
mod opcode;
mod processor;

fn main() {
    let bios = Bios::new("bios/SCPH1001.BIN");

    let bus = Bus::new(bios);

    let mut cpu = CPU::new(bus);

    cpu.run_next_instruction();

       
}


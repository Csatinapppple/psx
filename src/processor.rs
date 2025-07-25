mod consts;
mod bus;

use consts;
use bus::Bus;

struct CPU {
    bus: Bus,
    pc: usize      // Program Counter (PC)
}

impl CPU {
    
    pub fn run_next_instruction(&mut self) {
        let instruction = self.load32(self.pc);
        self.pc = self.pc.wrapping_add(4);
        self.decode_and_execute(instruction);
    }

    fn new(bus: Bus) -> Self {
        Self {
            bus: Bus,
            pc: consts::BIOS_START // Endereço inicial do BIOS do PS1
        }
    }
    
    fn load32(&self, addr: usize) -> u32 {
        self.bus.load32(addr)
    }
    
    fn decode_and_execute(&self, opcode: u32) {
        panic!("Unhandled_instruction_{:08x}", opcode);
    }

}


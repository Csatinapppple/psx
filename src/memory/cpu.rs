use crate::consts::BIOS_START;
use crate::memory::bus::Bus;
use crate::memory::instruction::Instruction;

pub struct CPU {
    bus: Bus,
    pc: usize, // Program Counter (PC)
    r: [u32; 32],
}

impl CPU {
    pub fn run_next_instruction(&mut self) {
        let instruction = Instruction(self.load32(self.pc));
        self.pc = self.pc.wrapping_add(4);
        self.decode_and_execute(instruction);
    }

    pub fn new(bus: Bus) -> Self {
        let registers: [u32; 32] = [0; 32];
        Self {
            bus: bus,
            pc: BIOS_START, // Endereço inicial do BIOS do PS1
            r: registers,
        }
    }

    fn load32(&self, addr: usize) -> u32 {
        self.bus.load32(addr)
    }

    pub fn decode_and_execute(&self, instruction: Instruction) {
        match instruction.primary() {
            0b001111 => self.op_lui(instruction),
            _ => panic!("Unhandled_instruction_{:08x}", instruction.get()),
        }
    }

    fn op_lui(&mut self, instruction: Instruction) {
        let i = instruction.imm();
        let t = instruction.t();

        let v = i << 16;

        self.set_r(t, v);
    }

    fn op_ori(&mut self, instruction: Instruction) {
        let i = instruction.imm();
        let t = instruction.t();
        let s = instruction.s();
    }

    fn set_r(&mut self, index: usize, val: u32) {
        if index > 0 {
            self.r[index] = val;
        }
    }
}

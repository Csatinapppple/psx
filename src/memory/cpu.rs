use crate::consts;
use crate::memory::bus::Bus;
use crate::memory::map::opcode;

pub struct CPU {
    bus: Bus,
    pc: usize, // Program Counter (PC)
    r: [u32; 32],
}

impl CPU {
    pub fn run_next_opcode(&mut self) {
        let opcode = self.load32(self.pc);
        self.pc = self.pc.wrapping_add(4);
        self.decode_and_execute(opcode);
    }

    pub fn new(bus: Bus) -> Self {
        let registers: [u32; 32] = [0; 32];
        Self {
            bus: bus,
            pc: consts::BIOS_START, // Endereço inicial do BIOS do PS1
            r: registers,
        }
    }

    fn load32(&self, addr: usize) -> u32 {
        self.bus.load32(addr)
    }

    fn store32(&mut self, addr: usize, val: u32) {
        self.bus.store32(addr, val);
    }

    pub fn decode_and_execute(&mut self, opcode: u32) {
        let primary = || opcode::PRIMARY.get(opcode);
        let rt = || opcode::RT.get(opcode) as usize;
        let rs = || opcode::RS.get(opcode) as usize;
        let imm = || opcode::IMM.get(opcode);

        match primary() {
            0b001111 => self.op_lui(imm(), rt()),
            0b001101 => self.op_ori(imm(), rt(), rs()),
            0b101011 => self.op_sw(imm(), rt(), rs()),
            _ => panic!("Unhandled_opcode::{:08x}", opcode),
        }
    }

    fn op_lui(&mut self, imm: u32, rt: usize) {
        let v = imm << 16;

        self.set_r(rt, v);
    }

    fn op_ori(&mut self, imm: u32, rt: usize, rs: usize) {
        let v = self.r[rs] | imm;
        self.set_r(rt, v);
    }

    // Store Word
    fn op_sw(&mut self, imm: u32, rt: usize, rs: usize) {
        let addr = self.r[rs].wrapping_add(imm) as usize;
        let v = self.r[rt];

        self.store32(addr, v);
    }

    fn set_r(&mut self, index: usize, val: u32) {
        if index > 0 {
            self.r[index] = val;
        }
    }
}

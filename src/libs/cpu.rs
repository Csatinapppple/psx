use crate::consts;
use crate::libs::bus::Bus;
use crate::libs::map::opcode;

pub struct CPU {
    bus: Bus,
    pc: u32, // Program Counter (PC)
    load: (usize, u32),
    r: [u32; 32],
    out_r: [u32; 32],
    next_opcode: u32,
    sr: u32, // Status Register
}

impl CPU {
    pub fn run_next_opcode(&mut self) {
        let opcode = self.next_opcode;

        let (reg, val) = self.load;
        self.set_r(reg, val);

        self.load = (0, 0);

        self.next_opcode = self.load32(self.pc as usize);

        self.pc = self.pc.wrapping_add(4);

        self.decode_and_execute(opcode);

        self.r = self.out_r;

        println!("curent opcode: {:08x}", opcode);
    }

    pub fn new(bus: Bus) -> Self {
        let registers: [u32; 32] = [0; 32];
        Self {
            bus: bus,
            pc: consts::BIOS_START as u32, // Endereço inicial do BIOS do PS1
            load: (0, 0),
            r: registers,
            out_r: registers,
            next_opcode: 0,
            sr: 0,
        }
    }

    fn load32(&self, addr: usize) -> u32 {
        self.bus.load32(addr)
    }

    fn store16(&mut self, addr: usize, val: u16) {
        self.bus.store16(addr, val);
    }

    fn store32(&mut self, addr: usize, val: u32) {
        self.bus.store32(addr, val);
    }

    pub fn decode_and_execute(&mut self, opcode: u32) {
        let primary = || opcode::PRIMARY.get(opcode);
        let secondary = || opcode::SECONDARY.get(opcode);
        let rt = || opcode::RT.get(opcode) as usize;
        let rs = || opcode::RS.get(opcode) as usize;
        let rd = || opcode::RD.get(opcode) as usize;
        let imm = || opcode::IMM.get(opcode);
        let imm_se = || {
            let imm_tmp = opcode::IMM.get(opcode) as i16;
            imm_tmp as u32
        };
        let imm5 = || opcode::IMM5.get(opcode);
        let imm_jmp = || opcode::IMM_JMP.get(opcode);

        match primary() {
            0x00 => match secondary() {
                0x00 => self.op_sll(imm5(), rt(), rd()),
                0x21 => self.op_addu(rt(), rs(), rd()),
                0x25 => self.op_or(rt(), rs(), rd()),
                0x2b => self.op_sltu(rt(), rs(), rd()),
                _ => panic!("unhandled_secondary_instruction_of_{:08x}", opcode),
            },
            0x02 => self.op_j(imm_jmp()),
            0x03 => self.op_jal(imm_jmp()),
            0x05 => self.op_bne(imm_se(), rt(), rs()),
            0x08 => self.op_addi(imm_se(), rt(), rs()),
            0x09 => self.op_addiu(imm_se(), rt(), rs()),
            0x0c => self.op_andi(imm(), rt(), rs()),
            0x0d => self.op_ori(imm(), rt(), rs()),
            0x0f => self.op_lui(imm(), rt()),
            0x10 => self.op_cop0(opcode),
            0x23 => self.op_lw(imm_se(), rt(), rs()),
            0x29 => self.op_sh(imm_se(), rt(), rs()),
            0x2b => self.op_sw(imm_se(), rt(), rs()),
            _ => panic!("Unhandled_opcode::{:08x}", opcode),
        }
    }
    

    fn op_sh(&mut self,imm_se: u32, rt: usize, rs: usize ){
        if self.sr & 0x10000 != 0{
            println!("Ignoring store while cache is isolated");
            return;
        }
        let v = (self.r[rt] & 0xffff) as u16;
        let i = self.r[rs].wrapping_add(imm_se) as usize;
        self.store16(i, v);
    }

    fn op_addu(&mut self, rt: usize, rs: usize, rd: usize) {
        let v = self.r[rs].wrapping_add(self.r[rt]);
        self.set_r(rd, v);
    }

    fn op_sltu(&mut self, rt: usize, rs: usize, rd: usize) {
        let v = self.r[rs] < self.r[rt];
        self.set_r(rd, v as u32);
    }

    fn op_lw(&mut self, imm_se: u32, rt: usize, rs: usize) {
        if self.sr & 0x10000 != 0 {
            println!("Ignoring load while cache is isolated");
            return;
        }

        let addr = self.r[rs].wrapping_add(imm_se);
        let v = self.load32(addr as usize);
        self.load = (rt, v);
    }

    fn op_addi(&mut self, imm_se: u32, rt: usize, rs: usize) {
        let imm_se = imm_se as i32;

        let s = self.r[rs] as i32;
        let v = match s.checked_add(imm_se) {
            Some(v) => v as u32,
            None => panic!("ADDI overflow"),
        };

        self.set_r(rt, v);
    }

    fn branch(&mut self, offset: u32) {
        let offset = offset << 2;
        self.pc = self.pc.wrapping_add(offset).wrapping_sub(4);
    }

    fn op_bne(&mut self, imm_se: u32, rt: usize, rs: usize) {
        if self.r[rs] != self.r[rt] {
            self.branch(imm_se);
        }
    }

    fn op_cop0(&mut self, opcode: u32) {
        let cop_rs = || opcode::RS.get(opcode) as usize;
        let cop_rt = || opcode::RT.get(opcode) as usize;
        let cop_rd = || opcode::RD.get(opcode) as usize;
        match cop_rs() {
            0b00100 => self.op_mtc0(cop_rt(), cop_rd()),
            _ => panic!("Unhandled cop0 instruction: {:08x}", opcode),
        }
    }

    fn op_mtc0(&mut self, rt: usize, rd: usize) {
        let v = self.r[rt];

        match rd {
            3 | 5 | 6 | 7 | 9 | 11 => {
                if v != 0 {
                    panic!("Unhandled write to cop0 {:}", rd);
                }
            }
            12 => self.sr = v,
            13 => {
                if v != 0 {
                    panic!("Unhandled write to cop0 {:}", rd);
                }
            }
            n => panic!("Unhandled cop0 register: {:08x}", n),
        }
    }

    fn op_jal(&mut self, imm_jmp: u32){
        let ra = self.pc;
        self.set_r(31, ra);
        self.op_j(imm_jmp);
    }

    fn op_j(&mut self, imm_jmp: u32) {
        self.pc = (self.pc & 0xf0000000) | (imm_jmp << 2);
    }

    fn op_addiu(&mut self, imm_se: u32, rt: usize, rs: usize) {
        let v = self.r[rs].wrapping_add(imm_se);
        self.set_r(rt, v);
    }

    fn op_sll(&mut self, imm5: u32, rt: usize, rd: usize) {
        let v = self.r[rt] << imm5;
        self.set_r(rd, v);
    }

    fn op_lui(&mut self, imm: u32, rt: usize) {
        let v = imm << 16;
        self.set_r(rt, v);
    }

    fn op_or(&mut self, rt: usize, rs: usize, rd: usize) {
        let v = self.r[rs] | self.r[rt];
        self.set_r(rd, v);
    }

    fn op_andi(&mut self, imm: u32, rt: usize, rs: usize) {
        let v = self.r[rs] & imm;
        self.set_r(rt, v);
    }

    fn op_ori(&mut self, imm: u32, rt: usize, rs: usize) {
        let v = self.r[rs] | imm;
        self.set_r(rt, v);
    }

    // Store Word
    fn op_sw(&mut self, imm_se: u32, rt: usize, rs: usize) {
        if self.sr & 0x10000 != 0 {
            println!("ignoring store while cache is isolated");
            return;
        }

        let addr = self.r[rs].wrapping_add(imm_se) as usize;
        let v = self.r[rt];

        self.store32(addr, v);
    }

    fn set_r(&mut self, index: usize, val: u32) {
        self.out_r[index] = val;
        self.out_r[0] = 0;
    }
}

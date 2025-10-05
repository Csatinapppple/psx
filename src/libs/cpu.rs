use crate::consts;
use crate::libs::bus::Bus;
use crate::libs::map::opcode;
use std::fmt;

pub struct CPU {
    bus: Bus,
    pc: u32, // Program Counter (PC)
    load: (usize, u32),
    r: [u32; 32],
    out_r: [u32; 32],
    opcode: u32,
    next_opcode: u32,
    sr: u32, // Status Register
    hi: u32,
    lo: u32,
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{
    program_counter: {:08x},
    load: (register: {:08x},value: {:08x}),
    opcode: {:08x},
    next_opcode: {:08x},
    status_register: {:08x},
}}",
            self.pc, self.load.0, self.load.1, self.opcode, self.next_opcode, self.sr
        )
    }
}

impl CPU {
    pub fn run_next_opcode(&mut self) {
        self.opcode = self.next_opcode;

        let (reg, val) = self.load;
        self.set_r(reg, val);

        self.load = (0, 0);

        self.next_opcode = self.load32(self.pc as usize);

        self.pc = self.pc.wrapping_add(4);

        self.decode_and_execute(self.opcode);

        self.r = self.out_r;
    }

    pub fn new(bus: Bus) -> Self {
        let registers: [u32; 32] = [0; 32];
        Self {
            bus: bus,
            pc: consts::BIOS_START as u32, // Endereço inicial do BIOS do PS1
            load: (0, 0),
            r: registers,
            out_r: registers,
            opcode: 0,
            next_opcode: 0,
            sr: 0,
        }
    }

    fn load32(&self, addr: usize) -> u32 {
        match self.bus.load32(addr) {
            Ok(integer) => return integer,
            Err(string) => panic!("{}", string),
        }
    }

    fn load8(&self, addr: usize) -> u8 {
        match self.bus.load8(addr) {
            Ok(integer) => return integer,
            Err(string) => panic!("{}", string),
        }
    }

    fn store8(&mut self, addr: usize, val: u8) {
        self.bus.store8(addr, val);
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
                0x03 => self.op_sra(imm5(), rt(), rd()),
                0x08 => self.op_jr(rs()),
                0x09 => self.op_jalr(rs(), rd()),
                0x20 => self.op_add(rt(), rs(), rd()),
                0x21 => self.op_addu(rt(), rs(), rd()),
                0x23 => self.op_subu(rt(), rs(), rd()),
                0x24 => self.op_and(rt(), rs(), rd()),
                0x25 => self.op_or(rt(), rs(), rd()),
                0x2b => self.op_sltu(rt(), rs(), rd()),
                _ => panic!(
                    "unhandled_secondary_instruction_of_{:08x}, CPU state {}",
                    opcode, self
                ),
            },
            0x01 => self.op_bxx(imm_se(), rt(), rs()),
            0x02 => self.op_j(imm_jmp()),
            0x03 => self.op_jal(imm_jmp()),
            0x04 => self.op_beq(imm_se(), rt(), rs()),
            0x05 => self.op_bne(imm_se(), rt(), rs()),
            0x06 => self.op_blez(imm_se(), rs()),
            0x07 => self.op_bgtz(imm_se(), rs()),
            0x08 => self.op_addi(imm_se(), rt(), rs()),
            0x09 => self.op_addiu(imm_se(), rt(), rs()),
            0x0a => self.op_slti(imm_se(), rt(), rs()),
            0x0c => self.op_andi(imm(), rt(), rs()),
            0x0d => self.op_ori(imm(), rt(), rs()),
            0x0f => self.op_lui(imm(), rt()),
            0x10 => self.op_cop0(opcode),
            0x20 => self.op_lb(imm_se(), rt(), rs()),
            0x23 => self.op_lw(imm_se(), rt(), rs()),
            0x24 => self.op_lbu(imm_se(), rt(), rs()),
            0x28 => self.op_sb(imm_se(), rt(), rs()),
            0x29 => self.op_sh(imm_se(), rt(), rs()),
            0x2b => self.op_sw(imm_se(), rt(), rs()),
            _ => panic!("Unhandled_opcode::{:08x}, CPU state: {}", opcode, self),
        }
    }

    fn op_sra(&mut self, imm5: u32, rt: usize, rd: usize) {
        let v = (self.r[rt] as i32) >> imm5;
        self.set_r(rd, v as u32);
    }

    fn op_subu(&mut self, rt: usize, rs: usize, rd: usize) {
        let v = self.r[rs].wrapping_sub(self.r[rt]);
        self.set_r(rd, v);
    }

    fn op_slti(&mut self, imm_se: u32, rt: usize, rs: usize) {
        let imm_se = imm_se as i32;
        let v = (self.r[rs] as i32) < imm_se;
        self.set_r(rt, v as u32);
    }

    // Encompasses BLTZ BGEZ BLTZAL BGEZAL
    fn op_bxx(&mut self, imm_se: u32, rt: usize, rs: usize) {
        let is_bgez = (rt & 1) as u32; //Branch if greater than or equal to zero
        let is_link = rt >> 1 == 0b1000;

        let v = self.r[rs] as i32;

        //branch if lower than zero
        let test = (v < 0) as u32;

        let test = test ^ is_bgez;
        //if test is true then its 1 and xors with is_bgez
        /*

           is_bgez | v < 0 | result
             0        1       1
             1        1       0
             0        0       0
             1        0       1

            if its BGEZ and greater than/eq zero it branches
            if its BLTZ then smaller than zero it branches
        */
        if is_link {
            let ra = self.pc;
            self.set_r(31, ra);
        }
        if test != 0 {
            self.branch(imm_se);
        }
    }

    fn op_jalr(&mut self, rs: usize, rd: usize) {
        let ra = self.pc;
        self.set_r(rd, ra);
        self.pc = self.r[rs];
    }

    fn op_lbu(&mut self, imm_se: u32, rt: usize, rs: usize) {
        let addr = self.r[rs].wrapping_add(imm_se) as usize;
        let v = self.load8(addr);
        self.load = (rt, v as u32);
    }

    fn op_blez(&mut self, imm_se: u32, rs: usize) {
        let v = self.r[rs] as i32;
        if v <= 0 {
            self.branch(imm_se);
        }
    }

    fn op_bgtz(&mut self, imm_se: u32, rs: usize) {
        let v = self.r[rs] as i32;
        if v > 0 {
            self.branch(imm_se);
        }
    }

    fn op_beq(&mut self, imm_se: u32, rt: usize, rs: usize) {
        if self.r[rs] == self.r[rt] {
            self.branch(imm_se);
        }
    }

    fn op_lb(&mut self, imm_se: u32, rt: usize, rs: usize) {
        let i = self.r[rs].wrapping_add(imm_se) as usize;
        let v = self.load8(i) as i8;

        self.load = (rt, v as u32);
    }

    fn op_jr(&mut self, rs: usize) {
        self.pc = self.r[rs];
    }

    fn op_sh(&mut self, imm_se: u32, rt: usize, rs: usize) {
        if self.sr & 0x10000 != 0 {
            println!("Ignoring store while cache is isolated");
            return;
        }
        let v = (self.r[rt] & 0xffff) as u16;
        let i = self.r[rs].wrapping_add(imm_se) as usize;

        self.store16(i, v);
    }

    fn op_sb(&mut self, imm_se: u32, rt: usize, rs: usize) {
        if self.sr & 0x10000 != 0 {
            // Cache is isolated, ignore write
            println!("Ignoring store while cache is isolated");
            return;
        }
        let v = (self.r[rt] & 0xff) as u8;
        let i = self.r[rs].wrapping_add(imm_se) as usize;

        self.store8(i, v);
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

    fn op_add(&mut self, rt: usize, rs: usize, rd: usize) {
        let s = self.r[rs] as i32;
        let t = self.r[rt] as i32;

        let v = match s.checked_add(t) {
            Some(v) => v as u32,
            None => panic!("ADD overflow"),
        };

        self.set_r(rd, v);
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
            0b00000 => self.op_mfc0(cop_rt(), cop_rd()),
            0b00100 => self.op_mtc0(cop_rt(), cop_rd()),
            _ => panic!(
                "Unhandled cop0 instruction: {:08x}\n CPU state: {}",
                opcode, self
            ),
        }
    }

    fn op_mfc0(&mut self, rt: usize, rd: usize) {
        let v = match rd {
            12 => self.sr,
            13 => panic!("unhandled read from CAUSE register"),
            _ => panic!("Unhandled read from cop0r{}", rd),
        };
        self.load = (rt, v);
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

    fn op_jal(&mut self, imm_jmp: u32) {
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

    fn op_and(&mut self, rt: usize, rs: usize, rd: usize) {
        let v = self.r[rs] & self.r[rt];
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

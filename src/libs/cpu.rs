use crate::consts;
use crate::libs::bus::Bus;
use crate::libs::map::opcode::Instruction;
use std::fmt;

pub struct CPU {
    bus: Bus,
    pc: u32, // Program Counter (PC)
    next_pc: u32,
    load: (usize, u32),
    r: [u32; 32],
    out_r: [u32; 32],
    opcode: Instruction,
    sr: u32, // Status Register
    hi: u32,
    lo: u32,
    current_pc: u32,
    cause: u32,
    epc: u32,
    branch: bool,
    delay_slot: bool,
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{
    program_counter: {:08x},
    next_program_counter: {:08x},
    opcode: {:08x},
    status_register: {:08x},
    hi: {:08x},
    lo: {:08x},
    current_pc {:08x},
    cause: {:08x},
    epc: {:08x},
    branch: {},
    delay_slot: {}
}}",
            self.pc,
            self.next_pc,
            self.opcode.0,
            self.sr,
            self.hi,
            self.lo,
            self.current_pc,
            self.cause,
            self.epc,
            self.branch,
            self.delay_slot
        )
    }
}

#[derive(Debug)]
enum Exception {
    LoadAddressError = 0x4,
    StoreAddressError = 0x5,
    SysCall = 0x8,
    Break = 0x9,
    Overflow = 0xc,
}

impl CPU {
    pub fn run_next_opcode(&mut self) {
        let (reg, val) = self.load;
        self.set_r(reg, val);

        self.load = (0, 0);

        self.current_pc = self.pc;

        if !Self::check_alignment(self.current_pc as usize, 4) {
            self.exception(Exception::LoadAddressError);
            return;
        }

        self.opcode = Instruction(self.load32(self.pc as usize));
        self.pc = self.next_pc;
        self.next_pc = self.pc.wrapping_add(4);

        self.delay_slot = self.branch;
        self.branch = false;

        self.decode_and_execute(self.opcode);

        self.r = self.out_r;
    }

    pub fn new(bus: Bus) -> Self {
        let registers: [u32; 32] = [0; 32];
        let start = consts::BIOS_START as u32;
        Self {
            bus: bus,
            pc: start, // Endereço inicial do BIOS do PS1
            next_pc: start.wrapping_add(4),
            load: (0, 0),
            r: registers,
            out_r: registers,
            opcode: Instruction(0),
            sr: 0,
            hi: 0,
            lo: 0,
            current_pc: 0,
            cause: 0,
            epc: 0,
            branch: false,
            delay_slot: false,
        }
    }

    fn check_alignment(addr: usize, alignment: usize) -> bool {
        addr % alignment == 0
    }

    fn load32(&self, addr: usize) -> u32 {
        return match self.bus.load32(addr) {
            Ok(val) => val,
            Err(string) => panic!("{}", string),
        };
    }

    fn load16(&self, addr: usize) -> u16 {
        return match self.bus.load16(addr) {
            Ok(val) => val,
            Err(string) => panic!("{}", string),
        };
    }

    fn load8(&self, addr: usize) -> u8 {
        return match self.bus.load8(addr) {
            Ok(val) => val,
            Err(string) => panic!("{}", string),
        };
    }

    fn store8(&mut self, addr: usize, val: u8) {
        self.bus
            .store8(addr, val)
            .unwrap_or_else(|string| panic!("{}", string));
    }

    fn store16(&mut self, addr: usize, val: u16) {
        self.bus
            .store16(addr, val)
            .unwrap_or_else(|string| panic!("{}", string));
    }

    fn store32(&mut self, addr: usize, val: u32) {
        self.bus
            .store32(addr, val)
            .unwrap_or_else(|string| panic!("{}", string));
    }

    pub fn decode_and_execute(&mut self, i: Instruction) {
        match i.primary() {
            0x00 => match i.secondary() {
                0x00 => self.op_sll(i.imm5(), i.rt(), i.rd()),
                0x02 => self.op_srl(i.imm5(), i.rt(), i.rd()),
                0x03 => self.op_sra(i.imm5(), i.rt(), i.rd()),
                0x04 => self.op_sllv(i.rt(), i.rs(), i.rd()),
                0x06 => self.op_srlv(i.rt(), i.rs(), i.rd()),
                0x07 => self.op_srav(i.rt(), i.rs(), i.rd()),
                0x08 => self.op_jr(i.rs()),
                0x09 => self.op_jalr(i.rs(), i.rd()),
                0x0c => self.exception(Exception::SysCall),
                0x0d => self.exception(Exception::Break),
                0x10 => self.op_mfhi(i.rd()),
                0x11 => self.op_mthi(i.rs()),
                0x12 => self.op_mflo(i.rd()),
                0x13 => self.op_mtlo(i.rs()),
                0x1a => self.op_div(i.rt(), i.rs()),
                0x1b => self.op_divu(i.rt(), i.rs()),
                0x18 => self.op_mult(i.rt(), i.rs()),
                0x19 => self.op_multu(i.rt(), i.rs()),
                0x20 => self.op_add(i.rt(), i.rs(), i.rd()),
                0x21 => self.op_addu(i.rt(), i.rs(), i.rd()),
                0x22 => self.op_sub(i.rt(), i.rs(), i.rd()),
                0x23 => self.op_subu(i.rt(), i.rs(), i.rd()),
                0x24 => self.op_and(i.rt(), i.rs(), i.rd()),
                0x25 => self.op_or(i.rt(), i.rs(), i.rd()),
                0x26 => self.op_xor(i.rt(), i.rs(), i.rd()),
                0x27 => self.op_nor(i.rt(), i.rs(), i.rd()),
                0x2b => self.op_sltu(i.rt(), i.rs(), i.rd()),
                0x2a => self.op_slt(i.rt(), i.rs(), i.rd()),
                _ => panic!(
                    "unhandled_secondary_instruction_of_{:08x}, CPU state {}",
                    i.0, self
                ),
            },
            0x01 => self.op_bxx(i.imm_se(), i.rt(), i.rs()),
            0x02 => self.op_j(i.imm_jmp()),
            0x03 => self.op_jal(i.imm_jmp()),
            0x04 => self.op_beq(i.imm_se(), i.rt(), i.rs()),
            0x05 => self.op_bne(i.imm_se(), i.rt(), i.rs()),
            0x06 => self.op_blez(i.imm_se(), i.rs()),
            0x07 => self.op_bgtz(i.imm_se(), i.rs()),
            0x08 => self.op_addi(i.imm_se(), i.rt(), i.rs()),
            0x09 => self.op_addiu(i.imm_se(), i.rt(), i.rs()),
            0x0a => self.op_slti(i.imm_se(), i.rt(), i.rs()),
            0x0b => self.op_sltiu(i.imm_se(), i.rt(), i.rs()),
            0x0c => self.op_andi(i.imm(), i.rt(), i.rs()),
            0x0d => self.op_ori(i.imm(), i.rt(), i.rs()),
            0x0f => self.op_lui(i.imm(), i.rt()),
            0x10 => self.op_cop0(i),
            0x20 => self.op_lb(i.imm_se(), i.rt(), i.rs()),
            0x21 => self.op_lh(i.imm_se(), i.rt(), i.rs()),
            0x23 => self.op_lw(i.imm_se(), i.rt(), i.rs()),
            0x24 => self.op_lbu(i.imm_se(), i.rt(), i.rs()),
            0x25 => self.op_lhu(i.imm_se(), i.rt(), i.rs()),
            0x28 => self.op_sb(i.imm_se(), i.rt(), i.rs()),
            0x29 => self.op_sh(i.imm_se(), i.rt(), i.rs()),
            0x2b => self.op_sw(i.imm_se(), i.rt(), i.rs()),
            _ => panic!("Unhandled_opcode::{:08x}, CPU state: {}", i.0, self),
        }
    }

    fn exception(&mut self, cause: Exception) {
        println!("Executing Exception: {:?}", cause);

        let handler = match self.sr & (1 << 22) != 0 {
            true => 0xbfc00180,
            false => 0x80000000,
        };

        let mode = self.sr & 0x3f;
        self.sr &= !0x3f;
        self.sr |= (mode << 2) & 0x3f;

        self.cause = (cause as u32) << 2;

        self.epc = self.current_pc;

        if self.delay_slot {
            self.epc = self.epc.wrapping_sub(4);
            self.cause |= 1 << 31;
        }

        self.pc = handler;
        self.next_pc = self.pc.wrapping_add(4);
    }

    fn op_mult(&mut self, rt: usize, rs: usize) {
        let a = (self.r[rs] as i32) as i64;
        let b = (self.r[rt] as i32) as i64;
        let v = (a * b) as u64;
        self.hi = (v >> 32) as u32;
        self.lo = v as u32;
    }

    fn op_multu(&mut self, rt: usize, rs: usize) {
        let a = self.r[rs] as u64;
        let b = self.r[rt] as u64;
        let v = a * b;
        self.hi = (v >> 32) as u32;
        self.lo = v as u32;
    }

    fn op_mtlo(&mut self, rs: usize) {
        self.lo = self.r[rs];
    }

    fn op_mthi(&mut self, rs: usize) {
        self.hi = self.r[rs];
    }

    fn op_nor(&mut self, rt: usize, rs: usize, rd: usize) {
        let v = !(self.r[rs] | self.r[rt]);
        self.set_r(rd, v);
    }

    fn op_xor(&mut self, rt: usize, rs: usize, rd: usize) {
        let v = self.r[rs] ^ self.r[rt];
        self.set_r(rd, v);
    }

    fn op_sllv(&mut self, rt: usize, rs: usize, rd: usize) {
        let v = self.r[rt] << (self.r[rs] & 0x1f);
        self.set_r(rd, v as u32);
    }

    fn op_slt(&mut self, rt: usize, rs: usize, rd: usize) {
        let rs = self.r[rs] as i32;
        let rt = self.r[rt] as i32;
        let v = rs < rt;
        self.set_r(rd, v as u32);
    }

    fn op_sltiu(&mut self, imm_se: u32, rt: usize, rs: usize) {
        let v = self.r[rs] < imm_se;
        self.set_r(rt, v as u32);
    }

    fn op_mflo(&mut self, rd: usize) {
        self.set_r(rd, self.lo);
    }

    fn op_mfhi(&mut self, rd: usize) {
        self.set_r(rd, self.hi);
    }

    fn op_divu(&mut self, rt: usize, rs: usize) {
        let num = self.r[rs];
        let div = self.r[rt];

        if div == 0 {
            self.hi = num;
            self.lo = 0xffff_ffff;
        } else {
            self.hi = num % div;
            self.lo = num / div;
        }
    }

    fn op_div(&mut self, rt: usize, rs: usize) {
        let num = self.r[rs] as i32;
        let div = self.r[rt] as i32;

        if div == 0 {
            self.hi = num as u32;
            self.lo = if num >= 0 { u32::MAX } else { 1 };
        } else if num as u32 == 0x8000_0000 && div == -1 {
            self.hi = 0;
            self.lo = 0x8000_0000;
        } else {
            self.hi = (num & div) as u32;
            self.lo = (num / div) as u32;
        }
    }

    fn op_srl(&mut self, imm5: u32, rt: usize, rd: usize) {
        let v = self.r[rt] >> imm5;
        self.set_r(rd, v);
    }

    fn op_srlv(&mut self, rt: usize, rs: usize, rd: usize) {
        let v = self.r[rt] >> (self.r[rs] & 0x1f);
        self.set_r(rd, v);
    }

    fn op_sra(&mut self, imm5: u32, rt: usize, rd: usize) {
        let v = (self.r[rt] as i32) >> imm5;
        self.set_r(rd, v as u32);
    }

    fn op_srav(&mut self, rt: usize, rs: usize, rd: usize) {
        let v = (self.r[rt] as i32) >> (self.r[rs] & 0x1f);
        self.set_r(rd, v as u32);
    }

    fn op_sub(&mut self, rt: usize, rs: usize, rd: usize) {
        let a = self.r[rs] as i32;
        let b = self.r[rt] as i32;
        match a.checked_sub(b) {
            Some(v) => self.set_r(rd, v as u32),
            None => self.exception(Exception::Overflow),
        };
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
            let ra = self.next_pc;
            self.set_r(31, ra);
        }
        if test != 0 {
            self.branch(imm_se);
        }
    }

    fn op_jalr(&mut self, rs: usize, rd: usize) {
        let ra = self.next_pc;
        self.set_r(rd, ra);
        self.branch = true;
        self.next_pc = self.r[rs];
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

    fn op_jr(&mut self, rs: usize) {
        self.branch = true;
        self.next_pc = self.r[rs];
    }

    fn op_addu(&mut self, rt: usize, rs: usize, rd: usize) {
        let v = self.r[rs].wrapping_add(self.r[rt]);
        self.set_r(rd, v);
    }

    fn op_sltu(&mut self, rt: usize, rs: usize, rd: usize) {
        let v = self.r[rs] < self.r[rt];
        self.set_r(rd, v as u32);
    }

    fn op_add(&mut self, rt: usize, rs: usize, rd: usize) {
        let s = self.r[rs] as i32;
        let t = self.r[rt] as i32;

        match s.checked_add(t) {
            Some(v) => self.set_r(rd, v as u32),
            None => self.exception(Exception::Overflow),
        };
    }

    fn op_addi(&mut self, imm_se: u32, rt: usize, rs: usize) {
        let imm_se = imm_se as i32;

        let s = self.r[rs] as i32;
        match s.checked_add(imm_se) {
            Some(v) => self.set_r(rt, v as u32),
            None => self.exception(Exception::Overflow),
        };
    }

    fn branch(&mut self, offset: u32) {
        let offset = offset << 2;
        self.branch = true;
        self.next_pc = self.next_pc.wrapping_add(offset).wrapping_sub(4);
    }

    fn op_bne(&mut self, imm_se: u32, rt: usize, rs: usize) {
        if self.r[rs] != self.r[rt] {
            self.branch(imm_se);
        }
    }

    fn op_cop0(&mut self, i: Instruction) {
        match i.rs() {
            0b00000 => self.op_mfc0(i.rt(), i.rd()),
            0b00100 => self.op_mtc0(i.rt(), i.rd()),
            0x10 => self.op_rfe(i),
            _ => panic!(
                "Unhandled cop0 instruction: {:08x}\n CPU state: {}",
                i.0, self
            ),
        }
    }

    fn op_rfe(&mut self, i: Instruction) {
        if i.secondary() != 0b010000 {
            panic!("Invalid cop0 instruction at rfe {:06b}", i.secondary());
        }
        let mode = self.sr & 0x3f;
        self.sr &= !0x3f;
        self.sr |= mode >> 2;
    }

    fn op_mfc0(&mut self, rt: usize, rd: usize) {
        let v = match rd {
            12 => self.sr,
            13 => self.cause,
            14 => self.epc,
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
        let ra = self.next_pc;
        self.set_r(31, ra);
        self.op_j(imm_jmp);
    }

    fn op_j(&mut self, imm_jmp: u32) {
        self.branch = true;
        self.next_pc = (self.next_pc & 0xf0000000) | (imm_jmp << 2);
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
    fn op_lb(&mut self, imm_se: u32, rt: usize, rs: usize) {
        let i = self.r[rs].wrapping_add(imm_se) as usize;
        let v = self.load8(i) as i8;

        self.load = (rt, v as u32);
    }

    fn op_lbu(&mut self, imm_se: u32, rt: usize, rs: usize) {
        let addr = self.r[rs].wrapping_add(imm_se) as usize;
        let v = self.load8(addr);
        self.load = (rt, v as u32);
    }

    fn op_lh(&mut self, imm_se: u32, rt: usize, rs: usize) {
        let addr = self.r[rs].wrapping_add(imm_se) as usize;

        if Self::check_alignment(addr, 2) {
            let v = self.load16(addr) as i16;
            self.load = (rt, v as u32);
        } else {
            self.exception(Exception::LoadAddressError);
        }
    }

    fn op_lhu(&mut self, imm_se: u32, rt: usize, rs: usize) {
        let addr = self.r[rs].wrapping_add(imm_se) as usize;

        if Self::check_alignment(addr, 2) {
            let v = self.load16(addr);
            self.load = (rt, v as u32);
        } else {
            self.exception(Exception::LoadAddressError);
        }
    }

    fn op_sh(&mut self, imm_se: u32, rt: usize, rs: usize) {
        if self.sr & 0x10000 != 0 {
            println!("Ignoring store while cache is isolated");
            return;
        }
        let v = (self.r[rt] & 0xffff) as u16;
        let i = self.r[rs].wrapping_add(imm_se) as usize;

        if Self::check_alignment(i, 2) {
            self.store16(i, v);
        } else {
            self.exception(Exception::StoreAddressError);
        }
    }

    fn op_lw(&mut self, imm_se: u32, rt: usize, rs: usize) {
        if self.sr & 0x10000 != 0 {
            println!("Ignoring load while cache is isolated");
            return;
        }

        let addr = self.r[rs].wrapping_add(imm_se) as usize;

        if Self::check_alignment(addr, 4) {
            let v = self.load32(addr);
            self.load = (rt, v);
        } else {
            self.exception(Exception::StoreAddressError);
        }
    }
    // Store Word
    fn op_sw(&mut self, imm_se: u32, rt: usize, rs: usize) {
        if self.sr & 0x10000 != 0 {
            println!("ignoring store while cache is isolated");
            return;
        }

        let addr = self.r[rs].wrapping_add(imm_se) as usize;
        let v = self.r[rt];

        if Self::check_alignment(addr, 4) {
            self.store32(addr, v);
        } else {
            self.exception(Exception::StoreAddressError);
        }
    }

    fn set_r(&mut self, index: usize, val: u32) {
        self.out_r[index] = val;
        self.out_r[0] = 0;
    }
}

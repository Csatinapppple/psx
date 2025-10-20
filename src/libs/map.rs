pub mod memory {
    use crate::consts;

    pub struct Range(usize, usize);

    impl Range {
        const REGION_MASK: [usize; 8] = [
            // KUSEG: 2048MB
            0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, // KSEG0:  512MB
            0x7fffffff, // KSEG1:  512MB
            0x1fffffff, // KSEG2: 1024MB
            0xffffffff, 0xffffffff,
        ];

        /// Mask a CPU address to remove the region bits.
        fn mask_region(addr: usize) -> usize {
            // Index address space in 512MB chunks
            let index = addr >> 29;

            addr & Self::REGION_MASK[index]
        }

        pub fn contains(self, offset: usize) -> Option<usize> {
            let offset = Self::mask_region(offset);
            let Range(start, length) = self;

            if offset >= start && offset < (start + length) {
                return Some(offset - start);
            } else {
                return None;
            }
        }
    }

    pub const BIOS: Range = Range(consts::BIOS_START, consts::BIOS_SIZE);
    pub const MEM_CONTROL: Range = Range(consts::HARDWARE_REGISTER_START, 36);
    pub const SYS_CONTROL: Range = Range(consts::SYS_CONTROL_START, 36);
    pub const RAM_SIZE: Range = Range(consts::RAM_SIZE_START, 4);
    pub const RAM: Range = Range(consts::RAM_START, consts::RAM_SIZE);
    pub const CACHE_CONTROL: Range = Range(consts::CACHE_CONTROL_START, 4);
    pub const SPU: Range = Range(consts::SPU_START, 640);
    pub const EXPANSION_1: Range = Range(consts::EXPANSION_1_START, 176);
    pub const EXPANSION_2: Range = Range(consts::EXPANSION_2_START, 66);
    pub const IRQ_CONTROL: Range = Range(consts::IRQ_START, 8);
    pub const TIMERS: Range = Range(consts::TIMER_REGISTER_START, 48);
    pub const DMA: Range = Range(consts::DMA_START, 0x80);
}

pub mod opcode {
    struct Range(usize, usize);

    /*
    Little-Endian bit table
    31-30-29-...-0
    */

    impl Range {
        fn get(&self, opcode: &Instruction) -> u32 {
            let Range(end, start) = self;

            let width = end - start + 1;
            let mask = (1 << width) - 1;

            (opcode.0 >> start) & mask
        }
    }

    #[derive(Copy, Clone)]
    pub struct Instruction(pub u32);

    impl Instruction {
        const PRIMARY: Range = Range(31, 26);
        const SECONDARY: Range = Range(5, 0);
        const RD: Range = Range(15, 11);
        const RT: Range = Range(20, 16);
        const IMM: Range = Range(15, 0);
        const IMM5: Range = Range(10, 6);
        const IMM_JMP: Range = Range(25, 0);
        const RS: Range = Range(25, 21);

        pub fn primary(&self) -> u32 {
            Self::PRIMARY.get(self)
        }

        pub fn secondary(&self) -> u32 {
            Self::SECONDARY.get(self)
        }

        pub fn rd(&self) -> usize {
            Self::RD.get(self) as usize
        }
        pub fn rt(&self) -> usize {
            Self::RT.get(self) as usize
        }
        pub fn rs(&self) -> usize {
            Self::RS.get(self) as usize
        }
        pub fn imm(&self) -> u32 {
            Self::IMM.get(self)
        }
        pub fn imm_se(&self) -> u32 {
            let imm_tmp = Self::IMM.get(self) as i16;
            imm_tmp as u32
        }
        pub fn imm5(&self) -> u32 {
            Self::IMM5.get(self)
        }
        pub fn imm_jmp(&self) -> u32 {
            Self::IMM_JMP.get(self)
        }
    }
}

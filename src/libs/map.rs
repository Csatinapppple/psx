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
        pub fn mask_region(addr: usize) -> usize {
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
}

pub mod opcode {
    pub struct Range(usize, usize);

    /*
    Little-Endian bit table
    31-30-29-...-0
    */

    impl Range {
        pub fn get(&self, opcode: u32) -> u32 {
            let Range(end, start) = self;

            let width = end - start + 1;
            let mask = (1 << width) - 1;

            (opcode >> start) & mask
        }
    }

    pub const PRIMARY: Range = Range(31, 26);
    pub const SECONDARY: Range = Range(5, 0);
    pub const RD: Range = Range(15, 11);
    pub const RT: Range = Range(20, 16);
    pub const IMM: Range = Range(15, 0);
    pub const IMM5: Range = Range(10, 6);
    pub const IMM_JMP: Range = Range(25, 0);
    pub const RS: Range = Range(25, 21);
}

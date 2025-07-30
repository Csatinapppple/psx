pub mod memory {
    use crate::consts;

    pub struct Range(usize, usize);

    impl Range {
        pub fn contains(self, offset: usize) -> Option<usize> {
            let Range(start, length) = self;

            if offset >= start && offset < start + length {
                return Some(offset - start);
            } else {
                return None;
            }
        }
    }

    pub const BIOS: Range = Range(consts::BIOS_START, consts::BIOS_SIZE);
    pub const MEM_CONTROL: Range = Range(consts::HARDWARE_REGISTER_START, 36);
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
    pub const RS: Range = Range(25, 21);
}

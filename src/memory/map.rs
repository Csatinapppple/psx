use crate::consts::{BIOS_SIZE, BIOS_START};

pub mod map {

    use crate::memory::map::{BIOS_SIZE, BIOS_START};

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
    pub const BIOS: Range = Range(BIOS_START, BIOS_SIZE);
}

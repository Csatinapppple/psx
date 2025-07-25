mod consts;

use consts::Consts;

mod map {
    struct Range(usize, usize);

    impl Range {
        pub fn contains(self, offset: usize) -> Option<usize> {
            let Range(start, length) = self;

            if addr >= start && addr < start + length {
                Some(addr - start)
            }
            None
        }   
    }

    pub const BIOS = Range(consts::BIOS_START, consts::BIOS_SIZE);
}

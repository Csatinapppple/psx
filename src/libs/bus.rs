use crate::libs::bios::Bios;
use crate::libs::map::memory;

pub struct Bus {
    bios: Bios,
}

impl Bus {
    fn check_alignment(addr: usize) {
        if addr % 4 != 0 {
            panic!("unhandled_unaligned_memory_access_at{:08x}", addr);
        }
    }

    pub fn new(bios: Bios) -> Self {
        Self { bios: bios }
    }

    pub fn load32(&self, addr: usize) -> u32 {
        Self::check_alignment(addr);

        if let Some(offset) = memory::BIOS.contains(addr) {
            return self.bios.load32(offset);
        }
        panic!("unhandled_load32_at_address_{:08x}", addr);
    }

    pub fn store32(&mut self, addr: usize, val: u32) {
        Self::check_alignment(addr);

        if let Some(offset) = memory::MEM_CONTROL.contains(addr) {
            match offset {
                0 => {
                    if val != 0x1f000000 {
                        panic!("bad_expansion_1_base_address:_0x{:08x}", val);
                    }
                }
                4 => {
                    if val != 0x1f802000 {
                        panic!("bad_expansion_2_base_address:_0x{:08x}", val);
                    }
                }
                _ => println!("Unhandled_write_to_MEM_CONTROL 0x{:08x}", val),
            }
            return;
        }

        panic!("unhandled_store32_into_address{:08x}", addr);
    }
}

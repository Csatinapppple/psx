use crate::memory::bios::Bios;
use crate::memory::map::map;

pub struct Bus {
    bios: Bios,
}

impl Bus {
    pub fn new(bios: Bios) -> Self {
        Self { bios: bios }
    }

    pub fn load32(&self, addr: usize) -> u32 {
        if let Some(offset) = map::BIOS.contains(addr) {
            return self.bios.load32(offset);
        }
        panic!("unhandled_load32_at_address_{:08x}", addr);
    }

    pub fn store32(&mut self, addr: usize, val: u32) {
        panic!("unhandled_store32_into_address{:08x}", addr);
    }
}

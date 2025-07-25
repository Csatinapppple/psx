pub struct Bus {
    bios: Bios
}

impl Bus {
    pub fn new(bios: Bios) -> Self {
        Self {
            bios: Bios
        }
    }

    pub fn load32(&self, addr: usize) -> u32{
        if let Some(offset) = map::BIOS.contains(addr) {
            self.bios.load32(offset)
        } 
        panic!("unhandled_load32_at_address_{:08x}", addr);
    }
}

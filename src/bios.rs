
use std::fs::File;

pub struct Bios{
    data: [consts::BIOS_SIZE, u8]
}

impl Bios {
    pub fn new(filename: &str) -> Self {
        let mut f = File::open(filename).expect("file not found");
        let mut buffer = [consts::BIOS_SIZE, u8];

        f.read(&mut buffer).expect("file couldn't be read");

        Self { data: buffer }
    }

    pub fn load32(&self, addr: usize) -> u32 {
        u32::from_le_bytes(self.data[addr..addr+4])
    }

}

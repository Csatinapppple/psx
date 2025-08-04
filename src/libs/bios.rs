use crate::consts;
use std::fs::File;
use std::io::Read;

pub struct Bios {
    data: Vec<u8>,
}

impl Bios {
    pub fn new(filename: &str) -> Self {
        let mut f = File::open(filename).expect("file not found");
        let mut buffer = vec![0; consts::BIOS_SIZE];

        f.read(&mut buffer).expect("file couldn't be read");

        Self { data: buffer }
    }

    pub fn load32(&self, addr: usize) -> u32 {
        let bytes: [u8; 4] = self.data[addr..addr + 4]
            .try_into()
            .expect("Failed to convert slice to array in Bios.rs");

        u32::from_le_bytes(bytes)
    }
}

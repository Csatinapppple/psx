use crate::consts;

pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    pub fn new() -> Ram {
        let data = vec![0xca; consts::RAM_SIZE];
        Self { data: data }
    }

    pub fn load8(&self, addr: usize) -> u8 {
        u8::from_le(self.data[addr])
    }

    pub fn store8(&mut self, addr: usize, val: u8) {
        self.data[addr] = val.to_le();
    }

    pub fn store16(&mut self, addr: usize, val: u16) {
        self.data[addr..addr + 2].copy_from_slice(&val.to_le_bytes());
    }

    pub fn load32(&self, addr: usize) -> u32 {
        let bytes: [u8; 4] = self.data[addr..addr + 4]
            .try_into()
            .expect("Failed to convert slice to array in Bios.rs");

        u32::from_le_bytes(bytes)
    }

    pub fn store32(&mut self, addr: usize, val: u32) {
        self.data[addr..addr + 4].copy_from_slice(&val.to_le_bytes());
    }
}

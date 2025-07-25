const BIOS_SIZE = 512 * 1024;

struct Bios{
    data: [u8; BIOS_SIZE]
}

impl Bios {
    pub fn new(filename: &str) -> Self {
        let mut f = File::open(filename).expect("file not found");
        let mut buffer = [u8; BIOS_SIZE];

        f.read(&mut buffer).expect("file couldn't be read");

        Bios { data: buffer }
    }

    pub fn load32(&self, addr: usize) -> u32 {
        u32::from_le_bytes(self.data[addr..addr+4])
    }
}

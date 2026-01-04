pub struct Dma {
    pub control: u32,
}

impl Dma {
    pub fn new() -> Dma {
        Dma {
            control: 0x07654321,
        }
    }
}

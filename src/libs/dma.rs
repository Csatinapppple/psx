pub struct Dma {
    pub control: u32,
    irq_en: bool,
    channel_irq_en: u8,
    channel_irq_flags: u8,
    force_irq: bool,
    irq_dummy: u8,
}

impl Dma {
    pub fn new() -> Dma {
        Dma {
            control: 0x07654321,
            irq_en: false,
            channel_irq_en: 0,
            channel_irq_flags: 0,
            force_irq: false,
            irq_dummy: 0,
        }
    }

    fn irq(&self) -> bool {
        let channel_irq = self.channel_irq_flags & self.channel_irq_en;
        self.force_irq || (self.irq_en && channel_irq != 0)
    }

    pub fn interrupt(&self) -> u32 {
        let mut r = 0;

        r |= self.irq_dummy as u32;
        r |= (self.force_irq as u32) << 15;
        r |= (self.channel_irq_en as u32) << 16;
        r |= (self.irq_en as u32) << 23;
        r |= (self.channel_irq_flags as u32) << 24;
        r |= (self.irq() as u32) << 31;
        r
    }

    pub fn set_interrupt(&mut self, val: u32) {
        self.irq_dummy = (val & 0x3f) as u8;
        self.force_irq = (val >> 15) & 1 != 0;
        self.channel_irq_en = ((val >> 16) & 0x7f) as u8;
        self.irq_en = (val >> 23) & 1 != 0;
        let ack = ((val >> 24) & 0x3f) as u8;
        self.channel_irq_flags &= !ack;
    }
}

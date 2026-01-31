#[derive(Copy, Clone)]
pub struct Channel {
    enable: bool,
    pub direction: Direction,
    pub step: Step,
    pub sync: Sync,
    trigger: bool,
    chop: bool,
    chop_dma_sz: u8,
    chop_cpu_sz: u8,
    dummy: u8,
    base: u32,
    block_size: u16,
    block_count: u16,
}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            enable: false,
            direction: Direction::ToRam,
            step: Step::Increment,
            sync: Sync::Manual,
            trigger: false,
            chop: false,
            chop_dma_sz: 0,
            chop_cpu_sz: 0,
            dummy: 0,
            base: 0,
            block_size: 0,
            block_count: 16,
        }
    }

    pub fn transfer_size(&self) -> Option<u32> {
        let bs = self.block_size as u32;
        let bc = self.block_count as u32;

        match self.sync {
            Sync::Manual => Some(bs),
            Sync::Request => Some(bc * bs),
            Sync::LinkedList => None,
        }
    }

    pub fn done(&mut self) {
        self.enable = false;
        self.trigger = false;
    }

    pub fn active(&self) -> bool {
        let trigger = match self.sync {
            Sync::Manual => self.trigger,
            _ => true,
        };
        self.enable && trigger
    }

    pub fn block_control(&self) -> u32 {
        let bs = self.block_size as u32;
        let bc = self.block_count as u32;

        (bc << 16) | bs
    }

    pub fn set_block_control(&mut self, val: u32) {
        self.block_size = val as u16;
        self.block_count = (val >> 16) as u16;
    }

    pub fn base(&self) -> u32 {
        self.base
    }
    pub fn set_base(&mut self, val: u32) {
        self.base = val & 0xffffff;
    }

    pub fn control(&self) -> u32 {
        let mut r = 0;

        r |= (self.direction as u32) << 0;
        r |= (self.step as u32) << 1;
        r |= (self.chop as u32) << 8;
        r |= (self.sync as u32) << 9;
        r |= (self.chop_dma_sz as u32) << 16;
        r |= (self.chop_cpu_sz as u32) << 20;
        r |= (self.enable as u32) << 24;
        r |= (self.trigger as u32) << 28;
        r |= (self.dummy as u32) << 29;

        r
    }

    pub fn set_control(&mut self, val: u32) {
        self.direction = match val & 1 != 0 {
            true => Direction::FromRam,
            false => Direction::ToRam,
        };

        self.step = match (val >> 1) & 1 != 0 {
            true => Step::Decrement,
            false => Step::Increment,
        };

        self.chop = (val >> 8) & 1 != 0;

        self.sync = match (val >> 9) & 3 {
            0 => Sync::Manual,
            1 => Sync::Request,
            2 => Sync::LinkedList,
            n => panic!("Unknown DMA sync mode {}", n),
        };

        self.chop_dma_sz = ((val >> 16) & 7) as u8;
        self.chop_cpu_sz = ((val >> 20) & 7) as u8;

        self.enable = (val >> 24) & 1 != 0;
        self.trigger = (val >> 28) & 1 != 0;

        self.dummy = ((val >> 29) & 3) as u8;
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    ToRam = 0,
    FromRam = 1,
}

#[derive(Copy, Clone)]
pub enum Step {
    Increment = 0,
    Decrement = 1,
}

#[derive(Copy, Clone)]
pub enum Sync {
    Manual = 0,
    Request = 1,
    LinkedList = 2,
}

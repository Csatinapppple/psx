use crate::libs::bios::Bios;
use crate::libs::map::memory;
use crate::libs::ram::Ram;

pub struct Bus {
    bios: Bios,
    ram: Ram,
}

impl Bus {
    pub fn new(bios: Bios, ram: Ram) -> Self {
        Self {
            bios: bios,
            ram: ram,
        }
    }

    pub fn load8(&self, addr: usize) -> Result<u8, String> {
        if let Some(offset) = memory::RAM.contains(addr) {
            return Ok(self.ram.load8(offset));
        } else if let Some(offset) = memory::BIOS.contains(addr) {
            return Ok(self.bios.load8(offset));
        } else if let Some(offset) = memory::EXPANSION_1.contains(addr) {
            println!("load8 at addr {:08x} EXPANSION_1", offset);
            return Ok(0xff);
        }

        Err(format!("unhandled load8 at address {:08x}", addr))
    }

    pub fn load32(&self, addr: usize) -> Result<u32, String> {
        if let Some(offset) = memory::RAM.contains(addr) {
            return Ok(self.ram.load32(offset));
        } else if let Some(offset) = memory::BIOS.contains(addr) {
            return Ok(self.bios.load32(offset));
        } else if let Some(offset) = memory::IRQ_CONTROL.contains(addr) {
            println!("IRQ Control read {:08x}", offset);
            return Ok(0);
        } else if let Some(offset) = memory::DMA.contains(addr) {
            println!("DMA read at: {:08x}", addr);
            return Ok(0);
        }

        Err(format!("unhandled_load32_at_address_{:08x}", addr))
    }

    pub fn store16(&mut self, addr: usize, val: u16) {
        if let Some(offset) = memory::SPU.contains(addr) {
            println!(
                "Unhandled write16 to SPU register {:x} with val {:04x}",
                offset, val
            );
            return;
        } else if let Some(offset) = memory::TIMERS.contains(addr) {
            println!(
                "Unhandled write16 to timer register {:08x} with val {:04x}",
                offset, val
            );
            return;
        } else if let Some(offset) = memory::RAM.contains(addr) {
            println!("Write of WORD at RAM {:08x} with val: {:04x}", offset, val);
            self.ram.store16(offset, val);
            return;
        }

        panic!(
            "unhandled store16 at addresss : Ox{:08x}. with val : {:016b} ",
            addr, val
        );
    }

    pub fn store8(&mut self, addr: usize, val: u8) {
        if let Some(offset) = memory::RAM.contains(addr) {
            self.ram.store8(offset, val);
            println!("Write of BYTE at RAM {:08x} with val: {:08b}", offset, val);
            return;
        } else if let Some(offset) = memory::EXPANSION_2.contains(addr) {
            println!(
                "Unhandled write of {:08b} to expansion 2 register {:x}",
                val, offset
            );
            return;
        }

        panic!("unhandled store8 into address {:08x}", addr);
    }

    pub fn store32(&mut self, addr: usize, val: u32) {
        if let Some(offset) = memory::RAM.contains(addr) {
            self.ram.store32(offset, val);
            return;
        } else if let Some(offset) = memory::MEM_CONTROL.contains(addr) {
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
        } else if let Some(offset) = memory::RAM_SIZE.contains(addr) {
            println!("ram_size_store at addr {:08x}", offset);
            return;
        } else if let Some(offset) = memory::CACHE_CONTROL.contains(addr) {
            println!("cache_control_store at addr {:08x}", offset);
            return;
        } else if let Some(offset) = memory::SYS_CONTROL.contains(addr) {
            println!("SYS_CONTROL__store at addr {:08x}", offset);
            return;
        } else if let Some(offset) = memory::IRQ_CONTROL.contains(addr) {
            println!("IRQ control: {:x} <- {:08x}", offset, val);
            return;
        } else if let Some(offset) = memory::DMA.contains(addr) {
            println!("DMA write at {:08x} with val {:08x}", offset, val);
            return;
        }

        panic!("unhandled_store32_into_address{:08x}", addr);
    }
}

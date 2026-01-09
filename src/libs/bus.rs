use crate::libs::bios::Bios;
use crate::libs::dma::{Dma, Port};
use crate::libs::map::memory;
use crate::libs::ram::Ram;

pub struct Bus {
    bios: Bios,
    ram: Ram,
    dma: Dma,
}

impl Bus {
    pub fn new(bios: Bios, ram: Ram) -> Self {
        Self {
            bios: bios,
            ram: ram,
            dma: Dma::new(),
        }
    }

    fn get_major_minor(offset: u32) -> (u32, u32) {
        ((offset & 0x70) >> 4, offset & 0xf)
    }

    fn dma_reg(&self, offset: usize) -> Result<u32, String> {
        let (major, minor) = Self::get_major_minor(offset as u32);
        let error = || {
            format!(
                "Unhandled DMA read at {:08x}, major: {}, minor: {}",
                offset, major, minor
            )
        };

        match major {
            0..=6 => {
                let channel = self.dma.channel(Port::from_index(major));

                match minor {
                    8 => Ok(channel.control()),
                    _ => Err(error()),
                }
            }
            7 => match minor {
                0 => Ok(self.dma.control),
                4 => Ok(self.dma.interrupt()),
                _ => Err(error()),
            },
            _ => Err(error()),
        }
    }

    fn set_dma_reg(&mut self, offset: usize, val: u32) -> Result<(), String> {
        let (major, minor) = Self::get_major_minor(offset as u32);
        let error = || {
            format!(
                "Unhandled DMA write at {:08x} with val {:08x}, major: {}, minor: {}",
                offset, val, major, minor
            )
        };

        match major {
            0..=6 => {
                let port = Port::from_index(major);
                let channel = self.dma.channel_mut(port);

                match minor {
                    8 => channel.set_control(val),
                    _ => return Err(error()),
                }
            }
            7 => match minor {
                0 => self.dma.control = val,
                4 => self.dma.set_interrupt(val),
                _ => return Err(error()),
            },
            _ => return Err(error()),
        };
        Ok(())
    }

    pub fn load8(&self, addr: usize) -> Result<u8, String> {
        if let Some(offset) = memory::RAM.contains(addr) {
            return Ok(self.ram.load8(offset));
        } else if let Some(offset) = memory::BIOS.contains(addr) {
            return Ok(self.bios.load8(offset));
        } else if let Some(offset) = memory::EXPANSION_1.contains(addr) {
            println!("load8 at addr {:08x} EXPANSION_1", addr);
            return Ok(0xff);
        }

        Err(format!("unhandled load8 at address {:08x}", addr))
    }

    pub fn load16(&self, addr: usize) -> Result<u16, String> {
        if let Some(offset) = memory::SPU.contains(addr) {
            println!("Unhandled load16 from SPU register {:08x}", offset);
            return Ok(0);
        } else if let Some(offset) = memory::RAM.contains(addr) {
            return Ok(self.ram.load16(offset));
        } else if let Some(offset) = memory::IRQ_CONTROL.contains(addr) {
            println!("IRQ control load16 at {:04x}", addr);
            return Ok(0);
        }

        Err(format!("Unhandled load16 at address {:08x}", addr))
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
            return self.dma_reg(offset);
        } else if let Some(offset) = memory::GPU.contains(addr) {
            println!("GPU read at: {:08x}", addr);
            return match offset {
                4 => Ok(0x1000_0000),
                _ => Ok(0),
            };
        }

        Err(format!("unhandled_load32_at_address_{:08x}", addr))
    }

    pub fn store16(&mut self, addr: usize, val: u16) -> Result<(), String> {
        if let Some(offset) = memory::SPU.contains(addr) {
            println!(
                "Unhandled write16 to SPU register {:x} with val {:04x}",
                offset, val
            );
            return Ok(());
        } else if let Some(offset) = memory::TIMERS.contains(addr) {
            println!(
                "Unhandled write16 to timer register {:08x} with val {:04x}",
                offset, val
            );
            return Ok(());
        } else if let Some(offset) = memory::RAM.contains(addr) {
            println!("Write of WORD at RAM {:08x} with val: {:04x}", offset, val);
            self.ram.store16(offset, val);
            return Ok(());
        } else if let Some(offset) = memory::IRQ_CONTROL.contains(addr) {
            println!("IRQ control store16: {:08x} <- {:04x}", addr, val);
            return Ok(());
        }

        Err(format!(
            "unhandled store16 at addresss : Ox{:08x}. with val : {:016b} ",
            addr, val
        ))
    }

    pub fn store8(&mut self, addr: usize, val: u8) -> Result<(), String> {
        if let Some(offset) = memory::RAM.contains(addr) {
            self.ram.store8(offset, val);
            println!("Write of BYTE at RAM {:08x} with val: {:08b}", offset, val);
            return Ok(());
        } else if let Some(offset) = memory::EXPANSION_2.contains(addr) {
            println!(
                "Unhandled write of {:08b} to expansion 2 register {:x}",
                val, offset
            );
            return Ok(());
        }

        Err(format!(
            "unhandled store8 into address {:08x} with val {:08b}",
            addr, val
        ))
    }

    pub fn store32(&mut self, addr: usize, val: u32) -> Result<(), String> {
        if let Some(offset) = memory::RAM.contains(addr) {
            self.ram.store32(offset, val);
            return Ok(());
        } else if let Some(offset) = memory::MEM_CONTROL.contains(addr) {
            match offset {
                0 => {
                    if val != 0x1f000000 {
                        return Err(format!("bad_expansion_1_base_address:_0x{:08x}", val));
                    }
                }
                4 => {
                    if val != 0x1f802000 {
                        return Err(format!("bad_expansion_2_base_address:_0x{:08x}", val));
                    }
                }
                _ => println!("Unhandled_write_to_MEM_CONTROL 0x{:08x}", val),
            }
            return Ok(());
        } else if let Some(offset) = memory::RAM_SIZE.contains(addr) {
            println!("ram_size_store at addr {:08x}", offset);
            return Ok(());
        } else if let Some(offset) = memory::CACHE_CONTROL.contains(addr) {
            println!("cache_control_store at addr {:08x}", offset);
            return Ok(());
        } else if let Some(offset) = memory::SYS_CONTROL.contains(addr) {
            println!("SYS_CONTROL__store at addr {:08x}", offset);
            return Ok(());
        } else if let Some(offset) = memory::IRQ_CONTROL.contains(addr) {
            println!("IRQ control: {:x} <- {:08x}", addr, val);
            return Ok(());
        } else if let Some(offset) = memory::DMA.contains(addr) {
            println!("DMA write at {:08x} with val {:08x}", addr, val);
            return self.set_dma_reg(offset, val);
        } else if let Some(offset) = memory::GPU.contains(addr) {
            println!("GPU write at {:08x} with val {:08x}", addr, val);
            return Ok(());
        } else if let Some(offset) = memory::TIMERS.contains(addr) {
            println!(
                "Unhandled store32 to timer register at {:08x} with val {:08x}",
                addr, val
            );
            return Ok(());
        }

        Err(format!(
            "unhandled_store32_into_address{:08x} with val {:08x}",
            addr, val
        ))
    }
}

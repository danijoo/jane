use crate::nes::ppubus::PPUMemory;
use crate::nes::types::*;
use image::{ImageBuffer, Rgba};
use palette::PALETTE;

pub mod palette;

pub type Pixel = Rgba<u8>;
pub type Sprite = ImageBuffer<Pixel, Vec<u8>>;

// PPU Control register flags
bitflags! {
    pub struct Control: Byte {
        const NAMETBL_X           = 1 << 0;
        const NAMETBL_Y           = 1 << 1;
        const INCREMENT_MODE      = 1 << 2;
        const PATTERN_SPRITE_ADDR = 1 << 3;
        const PATTERN_BG_ADDR     = 1 << 4;
        const SPRITE_SIZE         = 1 << 5;
        const SLAVE_MODE          = 1 << 6;
        const ENABLE_NMI          = 1 << 7;
    }
}

// PPU Mask register
bitflags! {
    pub struct Mask: Byte {
        const GRAYSCALE           = 1 << 0;
        const RENDER_BG_LEFT      = 1 << 1;
        const RENDER_SPRITES_LEFT = 1 << 2;
        const RENDER_BG           = 1 << 3;
        const RENDER_SPRITES      = 1 << 4;
        const ENHANCE_RED         = 1 << 5;
        const ENHANCE_GREEN       = 1 << 6;
        const ENHANCE_BLUE        = 1 << 7;
    }
}

bitflags! {
    pub struct Status: Byte {
        const SPRITE_OVERFOLW    = 1 << 5;
        const SPRITE_ZERO_HIT    = 1 << 6;
        const VERTICAL_BLANK     = 1 << 7;
    }
}

enum PPURegister {
    Control,
    Mask,
    Status,
    OAMAddr,
    OAMData,
    Scroll,
    Addr,
    Data,
    DMA 
}

pub struct Registers {
    // 0x2000
    pub ctrl: Control,
    // 0x2001
    pub mask: Mask,
    // 0x2002
    pub status: Status,
    // 0x2003
    pub oam_addr: Byte,
    // 0x2004
    pub oam_data: Byte,
    // 0x2005
    pub scroll: Byte,
    // 0x2006
    pub addr: Addr,
    // 0x2007
    pub data: Byte, 
    // 0x2008
    pub dma: Byte,  // 0x4014 
}

impl Registers {
    fn new() -> Registers {
        Registers {
           ctrl: Control::from_bits(0x00).unwrap(),
           mask: Mask::from_bits(0x00).unwrap(),
           status: Status::from_bits(0x00).unwrap(),
           oam_addr: 0x00,
           oam_data: 0x00,
           scroll: 0x00,
           addr: 0x00,
           data: 0x00,
           dma: 0x00, 
        }
    }
}


pub struct PPU {
    pub regs: Registers,
    pub cycle: u16, 
    pub scanline: u16,
    pub frame_ready: bool,
    pub nmi: bool,
    pub canvas_main: Sprite,
    pub pattern_tables: [Sprite; 2],
    pub palettes: [Sprite; 8],
    addr_latch_set: bool,
    data_buffer: Byte,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            regs: Registers::new(),
            cycle: 0,
            scanline: 0,
            frame_ready: false,
            nmi: false,
            canvas_main: ImageBuffer::from_pixel(256, 240, PALETTE[&0x00]),
            pattern_tables: [
                ImageBuffer::from_pixel(128, 128, PALETTE[&0x00]),
                ImageBuffer::from_pixel(128, 128, PALETTE[&0x00])
                ],
            palettes: [
                ImageBuffer::from_pixel(4, 1, PALETTE[&0x00]),
                ImageBuffer::from_pixel(4, 1, PALETTE[&0x00]),
                ImageBuffer::from_pixel(4, 1, PALETTE[&0x00]),
                ImageBuffer::from_pixel(4, 1, PALETTE[&0x00]),
                ImageBuffer::from_pixel(4, 1, PALETTE[&0x00]),
                ImageBuffer::from_pixel(4, 1, PALETTE[&0x00]),
                ImageBuffer::from_pixel(4, 1, PALETTE[&0x00]),
                ImageBuffer::from_pixel(4, 1, PALETTE[&0x00]),
            ], 
            addr_latch_set: false,
            data_buffer: 0
        }
    }

    pub fn reset(&mut self) {
        self.regs = Registers::new();
    }

    fn set_status(&mut self, flag: Status, val: bool) {
        if val {
            self.regs.status |= flag;
        } else {
            self.regs.status &= !flag;
        }
    }

    fn get_status(&self, flag: Status) -> bool {
        self.regs.status.contains(flag)
    }

    fn set_control(&mut self, flag: Control, val: bool) {
        if val {
            self.regs.ctrl |= flag;
        } else {
            self.regs.ctrl &= !flag;
        }
    }

    fn get_control(&self, flag: Control) -> bool {
        self.regs.ctrl.contains(flag)
    }

    

    // PPU renders 262 scanlines with 341 clocks per line. One px per clock
    // Scanline -1,261: Dummy scanline
    // Scanline 0-239: Visible scanlines:
    //      Cycle 0: idle.
    //      Cycle 1-256: Fetch tile data
    //      Cycle 257-320: Fetch tile data of sprites for next scanline 
    //      Cycle 321-336: Fetch first two tiles of next scanline
    //      Cycle 337-340: "Unknown" data fetch  
    // Scanline 240: PPU idle
    // Scanline 241-260: Vblack. Flag is set during second clock of 241 together
    // with NMI 
    pub fn clock<T: PPUMemory>(&mut self, _mem: &mut T) {
        if self.cycle == 340 {
            self.cycle = 0;
            if self.scanline == 261 {
                self.scanline = 0;
                self.frame_ready = true;
            } else {
                self.scanline += 1;
            }
        } else {
            self.cycle += 1;
        };

        // set/clear vblank flag
        if self.scanline == 241 && self.cycle == 1 {
            self.set_status(Status::VERTICAL_BLANK, true);
            if self.get_control(Control::ENABLE_NMI) {
                self.nmi = true;
            }

        } else if self.scanline == 261 && self.cycle == 1 {
            self.set_status(Status::VERTICAL_BLANK, false);
        }
    }

    // read from the main bus
    pub fn readb<T: PPUMemory>(&mut self, mem: &T, addr: Addr) -> Byte {
       // Only certain registers of the PPU can actually by read
       // remaining registers and read attemps will return garbage
        match addr {
            // status
            0x2002 => {
                let status = self.regs.status.bits();
                // Reading the status register also clears VBLANK and the
                // address latch
                self.set_status(Status::VERTICAL_BLANK, false);
                self.addr_latch_set = false;
                status
            },
            // oam data 
            0x2004 => { /* TODO */ 0x00 },
            // ppu data
            0x2007 => { 
                // ppu reads are delayed by one clock. Therefore, this uses
                // a buffer variable to return the data from the previous
                // read, and then set the new data to the buffer. However,
                // because the PPU is weird, this does not apply for the 
                // palette memory
                let data = self.data_buffer;
                self.data_buffer = mem.readb_ppu(addr);

                if addr > 0x3F00 {  // everything above 0x3F00 is palette
                   self.data_buffer 
                } else {
                    data
                }
            },
            _ => 0x00,  // unmapped reads                      
        } 
    }

    // // write to the main bus
    pub fn writeb<T: PPUMemory>(&mut self, mem: &mut T, addr: Addr, data: Byte) {
        // Only some of the PPU regs can be written to
        match addr {
            // Control 
            0x2000 => { 
                self.regs.ctrl = Control::from_bits(data).unwrap()
            },
            // Mask 
            0x2001 => {
                self.regs.mask = Mask::from_bits(data).unwrap()
            },
            // OAM address
            0x2003 => { /* TODO */ },
            // OAM data
            0x2004 => { /* TODO */ },
            // Scroll
            0x2005 => { /* TODO */ },
            // Addr
            0x2006 => {
                // To write a 16bit addr to the ppu, two consecutive writes are 
                // required to set the hi and lo byte of the address.
                // addr_latch_set indicates wether the hi byte is already
                // set or not
                if !self.addr_latch_set {
                    self.regs.addr = self.regs.addr & 0x00FF | (data as Word) << 8;
                } else {
                    self.regs.addr = self.regs.addr & 0xFF00 | data as Word;
                }
                self.addr_latch_set = !self.addr_latch_set;
            }
            // write data to the ppu addr bus
            0x2007 => {
                mem.writeb_ppu(self.regs.addr, data);
                // after write, increment vram addr for next write.
                // The increment value is determined by the vertical mode
                // flag of the status reg 0: +1, 1: +32
                self.regs.addr += if self.get_control(Control::INCREMENT_MODE) {
                    32 
                } else {
                    1
                }
            },
            _ => { } // unwriteable addr, do nothing
        }
    }

    // get a colored pixel using the NES color palette for given palette_id
    // and pixel value
    fn get_color_from_ram<T: PPUMemory>(&self, mem: &T, palette_id: u8, pixel: u8) -> Pixel {
        // 0x3F00: Start of palette memory
        // palette << 2: Palette size is 4
        // pixel: pixel index is 0,1,2 or 3
        // 0x3F (63): limits reading to PALETTE size
        let palette_idx_addr = 0x3F00 + ((palette_id as Word) << 2) + pixel as Word;
        // println!(palette_id_addr);
        let palette_idx = mem.readb_ppu(palette_idx_addr) & 0x3F;
        PALETTE[&palette_idx]
    }

    // updates the palette from VRAM and returns a sprite with the 4 colors
    pub fn get_palette<T: PPUMemory>(&mut self, mem: &T, palette_id: u8) -> &Sprite {
        for i in 0..4 {
            self.palettes[palette_id as usize].put_pixel(
                i, 0,
                self.get_color_from_ram(mem, palette_id, i as u8)
                );
        }
        &self.palettes[palette_id as usize]
    }

    // Get the correct color for the pixel from the given palette
    fn get_color(&self, pixel: Byte, _palette: Byte) -> Pixel {
        // TODO: not implemented yet. returns some black and white color
        match pixel {
            0 => PALETTE[&0x30],
            1 => PALETTE[&0x3d],
            2 => PALETTE[&0x2d],
            _ => PALETTE[&0x3f]
        }
    }

    // Get one of the two pattern tables of the PPU
    // This also initializes/updates the pattern table
    pub fn get_pattern_table<T: PPUMemory>(&mut self, mem: &T, index: usize, palette_id: Byte) -> &Sprite {
        // 16 x 16 tiles of 8x8px sprites per pattern table => 128x128px
        for x in 0..16 {  // tile row
            for y in 0..16 {  // tile column
                // byte offset in pattern mem. Each row is 256 bytes
                // and each pixel is 16 bytes
                let tile_offset_b = y*256 + x*16;  

                // iterate over individual pixels of one tile
                for row in 0..8 { 
                    let tile_addr = index as Addr * 0x1000 + tile_offset_b + row;

                    // NES memory organization: The pattern table defines the
                    // two least significant bits of the color (value between
                    // 0-3). Two bitplanes in memory each having one byte per
                    // row 
                    let mut tile_lsb = mem.readb_ppu(tile_addr);
                    let mut tile_msb = mem.readb_ppu(tile_addr + 8);
                    for col in 0..8 {
                        let pixel = (tile_lsb & 0x01) + (tile_msb & 0x01);
                        self.pattern_tables[index].put_pixel(
                            (x * 8 + (7-col)) as u32, // x starts on the right, sprit is from left
                            (y * 8 + row) as u32,
                            self.get_color_from_ram(mem, palette_id, pixel));
                        tile_lsb >>= 1;
                        tile_msb >>= 1;
                    } 
                } 
            }
        }

        &self.pattern_tables[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nes::ppubus::PPUBus;

    #[test]
    fn test_set_flags() {
        let mut ppu = PPU::new();

        // set unset flag
        assert_eq!(ppu.regs.status.bits(), 0b00000000);
        ppu.set_status(Status::VERTICAL_BLANK, true);
        assert_eq!(ppu.regs.status.bits(), 0b10000000,
            "register={:#010b}; should be 0b10000000", ppu.regs.status.bits());
        ppu.set_status(Status::VERTICAL_BLANK, false);
        assert_eq!(ppu.regs.status.bits(), 0b00000000,
            "register={:#010b}; should be 0b00000000", ppu.regs.status.bits());


        // set same flag twice
        ppu.set_status(Status::VERTICAL_BLANK, true);
        assert_eq!(ppu.regs.status.bits(), 0b10000000,
            "register={:#010b}; should be 0b10000000", ppu.regs.status.bits());
        ppu.set_status(Status::VERTICAL_BLANK, true);
        assert_eq!(ppu.regs.status.bits(), 0b10000000,
            "register={:#010b}; should be 0b10000000", ppu.regs.status.bits());


        // set other flag
        ppu.set_status(Status::SPRITE_ZERO_HIT, true);
        assert_eq!(ppu.regs.status.bits(), 0b11000000,
            "register={:#010b}; should be 0b11000000", ppu.regs.status.bits());

        ppu.set_status(Status::SPRITE_ZERO_HIT, false);
        assert_eq!(ppu.regs.status.bits(), 0b10000000,
            "register={:#010b}; should be 0b10000000", ppu.regs.status.bits());


        // set other register
        ppu.set_control(Control::INCREMENT_MODE, true);
        assert_eq!(ppu.regs.status.bits(), 0b10000000,
            "register={:#010b}; should be 0b10000000", ppu.regs.status.bits());
        assert_eq!(ppu.regs.ctrl.bits(), 0b0000100,
            "register={:#010b}; should be 0b0000001", ppu.regs.ctrl.bits());
    }

    #[test]
    fn test_write_addr() {
        let mut ppu = PPU::new();
        let mut ppu_bus = PPUBus::new();
        assert_eq!(ppu.regs.addr, 0x0000);
        ppu.writeb(&mut ppu_bus, 0x2006, 0x12);
        assert_eq!(ppu.regs.addr, 0x1200);
        ppu.writeb(&mut ppu_bus, 0x2006, 0x34);
        assert_eq!(ppu.regs.addr, 0x1234);
        ppu.writeb(&mut ppu_bus, 0x2006, 0x56);
        assert_eq!(ppu.regs.addr, 0x5634);
    }
}

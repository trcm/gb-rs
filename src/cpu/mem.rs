pub struct MMU {
    boot: [u8; 0xffff],
}

impl MMU {

    // [0000-3FFF] Cartridge ROM, bank 0
    // [0000-00FF] BIOS
    // [0100-014F] Cartridge header
    // [4000-7FFF] Cartridge ROM, other banks
    // [8000-9FFF] Graphics RAM
    // [A000-BFFF] Cartridge (External) RAM
    // [C000-DFFF] Working RAM
    // [E000-FDFF] Working RAM (shadow)
    // [FE00-FE9F] Graphics
    // [FF00-FF7F] Memory-mapped I/O
    // [FF80-FFFF] Zero-page RAM

    // 0xFF40 - LCD Control Register
    // Bit 7 - LCD Power (0=Off, 1=On)
    // Bit 6 - Window Tile Map (0=9800h-9BFFh, 1=9C00h-9FFFh)
    // Bit 5 - Window Enable (0=Disabled, 1=Enabled)
    // Bit 4 - BG & Window Tileset (0=8800h-97FFh, 1=8000h-8FFFh)
    // Bit 3 - BG Tile Map (0=9800h-9BFFh, 1=9C00h-9FFFh)
    // Bit 2 - Sprite Size (0=8×8, 1=8×16)
    // Bit 1 - Sprites Enabled (0=Disabled, 1=Enabled)
    // Bit 0 - BG Enabled (in DMG) (0=Disabled, 1=Enabled)
    // 0xFF41 - LCD Status 
    // Bit 6 - LYC Check
    // Bit 5 - Mode 2 OAM Checj
    // Bit 4 - Mode 1 V Blank check 
    // Bit 3 - Mode 0 H Blank Check
    // Bit 2 - LYC Comp signal
    // Bit 1/0 - Screen mode
    // 0: H blank
    // 1: V blank
    // 2: Searching OAM
    // 3: Transfer data to lcd
    
    pub fn new() -> MMU {
        MMU {
            boot: [0; 0xffff]
        }
    }

    pub fn read_value_u8(&self, location: usize) -> u8 {
        self.boot[location]
    }
    
    pub fn load_value_u8(&mut self, location: usize, value: u8) {
        if location == 0xFF40 {
            panic!("LCD Control");
        }
        if location == 0xFF41 {
            panic!("LCD STATUS");
        }
        self.boot[location] = value;
        // println!("Loaded\n Value: {:X} Location: {:X}, value: {:X}", value, location, self.boot[location]);
    }

    // pub fn load_value_u16(&mut self, location: usize, value: u16) {
    //     if location == 0xFF40 {
    //         panic!("LCD Control");
    //     }
    //     if location == 0xFF41 {
    //         panic!("LCD STATUS");
    //     }
    //     if location == 0xFF47 {
    //         panic!("HERE");
    //     }
    //     let upper: u8 = (value >> 8) as u8;
    //     let lower: u8 = (value & 0xFF00) as u8;
    //     self.boot[location] = upper;
    //     self.boot[location + 1] = lower;
    // }
}

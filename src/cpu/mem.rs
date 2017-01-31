pub struct MMU {
    boot: [u8; 0xffff],
}

impl MMU {

    pub fn new() -> MMU {
        MMU {
            boot: [0; 0xffff]
        }
    }

    pub fn load_value_u8(&mut self, location: usize, value: u8) {
        self.boot[location] = value;
        // println!("Loaded\n Value: {:X} Location: {:X}, value: {:X}", value, location, self.boot[location]);
    }

    pub fn load_value_u16(&mut self, location: usize, value: u16) {
        let upper: u8 = (value >> 8) as u8;
        let lower: u8 = (value & 0xFF00) as u8;
        self.boot[location] = upper;
        self.boot[location + 1] = lower;
    }
}

use cpu::cpu::CPU;

pub struct Debug<'a> {
    pub location: u16,
    pub cpu: &'a CPU,
}

impl<'a> Debug<'a> {
    pub fn new(cpu: &CPU) -> Debug {
        let mut debugger = Debug {
            location: 0,
        };
        debugger.location = cpu.pc;
        debugger
    }

    pub fn breakpoint() {

    }

    pub fn disassemble(&self, value: u8) {
        
    }

    pub fn step(&self) {
        self.cpu.cycle();
    }
    
    pub fn print_status(&self) {
        println!("DEBUGGER PC: {}", self.cpu.pc);
        println!("{:?}", self.cpu);
    }
}

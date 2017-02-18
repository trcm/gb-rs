use std::fmt;

pub struct Command {
    String: name,
    String: debug_msg
}


const OPCODE_DEC_B: u16 = 0x05;
const OPCODE_LD_B: u16 = 0x06;
const OPCODE_INC_C: u16 = 0x0C;


pub enum Opcode {
    DecB,
    Ldb,
    IncC
}

impl Opcode {
    pub fn parse(bits: u16) -> Opcode {
        match bits {
            
        }
    }
}

impl fmt::Display for Opcoe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = match self {
            &Opcode::DecB => "DEC B",
            &Opcode::Ldb => "LD B, d8",
            &Opcode::IncC => "INC C",
        };

        write!(f, "{}", code);
    }
}

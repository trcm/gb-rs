use std::fmt;

pub struct Command {
    String: name,
    String: debug_msg
}


const OPCODE_DEC_B: u16 = 0x05;
const OPCODE_LD_B: u16 = 0x06;
const OPCODE_INC_C: u16 = 0x0C;
const OPCODE_LD_DE: u16 = 0x17;




pub enum Opcode {
    DecB,
    Ldb,
    IncC,
    Ldde,
}

impl Opcode {
    pub fn parse(bits: u16) -> Opcode {
        match bits {
            OPCODE_DEC_B => Opcode::DecB,
            OPCODE_LD_B => Opcode::Ldb,
            OPCODE_INC_C => Opcode::IncC,
            OPCODE_Ldde => Opcode::Ldde,
        }

    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = match self {
            &Opcode::DecB => "DEC B",
            &Opcode::Ldb => "LD B, d8",
            &Opcode::IncC => "INC C",
        };

        write!(f, "{}", code);
    }
}

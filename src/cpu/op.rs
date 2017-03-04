use std::fmt;

const OPCODE_DEC_B: u8       = 0x05;
const OPCODE_LD_B: u8        = 0x06;
const OPCODE_INC_C: u8       = 0x0C;
const OPCODE_INC_DE: u8      = 0x13;
const OPCODE_LD_DE: u8       = 0x11;
const OPCODE_RLA: u8         = 0x17;
const OPCODE_LD_A_DE: u8     = 0x1A;
const OPCODE_JR_NZ: u8       = 0x20;
const OPCODE_LD_HL: u8       = 0x21;
const OPCODE_LD_SP: u8       = 0x31;
const OPCODE_LD_HL_DEC_A: u8 = 0x32;
const OPCODE_LD_HL_ADD_A: u8 = 0x22;
const OPCODE_INC_HL: u8      = 0x23;
const OPCODE_LD_A: u8        = 0x3E;
const OPCODE_XOR_A: u8       = 0xAF;
const OPCODE_LD_C_A: u8      = 0x4F;
const OPCODE_POP_BC: u8      = 0xC1;
const OPCODE_PUSH_BC: u8     = 0xC5;
const OPCODE_CALL_NN: u8     = 0xCD;
const OPCODE_LD_C: u8        = 0x0E;
const OPCODE_LDH_A:u8        = 0xE0;
const OPCODE_LD_C_ADD_A:u8   = 0xE2;
const OPCODE_LD_HL_A: u8     = 0x77;

const OPCODE_CB: u8          = 0xCB;
const OPCODE_CB_7C: u8       = 0x7C;
const OPCODE_CB_RLC: u8      = 0x11;

const OPCODE_RET: u8         = 0xC9;

pub enum Opcode {
    DecB,
    LdB,
    IncC,
    IncDE,
    LdDE,
    RLA,
    LdADE,
    JRNZ,
    LdHL,
    LdSP,
    LdHLDECA,
    LdHLADDA,
    IncHL,
    LdA,
    XORA,
    LdCA,
    PopBC,
    PushBC,
    CallNN,
    LdhA,
    LdC,
    LdCADDA,
    LdHLA,
    CB,
    CB7C,
    CBRLC,
    RET,
}

impl Opcode {
    pub fn parse(bits: u8) -> Opcode {
        match bits {
            OPCODE_DEC_B       => Opcode::DecB,
            OPCODE_LD_B        => Opcode::LdB,
            OPCODE_INC_C       => Opcode::IncC,
            OPCODE_LD_DE       => Opcode::LdDE,
            OPCODE_RLA         => Opcode::RLA,
            OPCODE_JR_NZ       => Opcode::JRNZ,
            OPCODE_LD_SP       => Opcode::LdSP,
            OPCODE_LD_HL       => Opcode::LdHL,
            OPCODE_LD_HL_DEC_A => Opcode::LdHLDECA,
            OPCODE_LD_HL_ADD_A => Opcode::LdHLADDA,
            OPCODE_INC_HL      => Opcode::IncHL,
            OPCODE_INC_DE      => Opcode::IncDE,
            OPCODE_LD_A        => Opcode::LdA,
            OPCODE_XOR_A       => Opcode::XORA,
            OPCODE_LD_C_A      => Opcode::LdCA,
            OPCODE_POP_BC      => Opcode::PopBC,
            OPCODE_PUSH_BC     => Opcode::PushBC,
            OPCODE_CALL_NN     => Opcode::CallNN,
            OPCODE_LDH_A       => Opcode::LdhA,
            OPCODE_LD_C_ADD_A  => Opcode::LdCADDA,
            OPCODE_LD_HL_A     => Opcode::LdHLA,
            OPCODE_LD_C        => Opcode::LdC,
            OPCODE_CB          => Opcode::CB,
            OPCODE_LD_A_DE     => Opcode::LdADE,
            OPCODE_RET         => Opcode::RET,
            _                  => panic!("Unimplemented opcode 0x{:02X}", bits)
        }
    }

    pub fn redirect(bits: u8) -> Opcode {
        match bits {
            OPCODE_CB_RLC => Opcode::CBRLC,
            OPCODE_CB_7C => Opcode::CB7C,
            _ => panic!("Unimplemented redirect")
        }
    }

}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = match self {
            &Opcode::DecB     => "DEC B",
            &Opcode::LdB      => "LD B, d8",
            &Opcode::IncC     => "INC C",
            &Opcode::LdDE     => "LD DE, d16",
            &Opcode::RLA      => "RLA",
            &Opcode::JRNZ     => "JR NZ",
            &Opcode::LdSP     => "LD SP, d16",
            &Opcode::LdHL     => "LD HL, (d16)",
            &Opcode::LdADE    => "LD A, (DE)",
            &Opcode::LdHLDECA => "LD (HL-), A",
            &Opcode::LdHLADDA => "LD (HL+), A",
            &Opcode::IncHL    => "INC HL",
            &Opcode::IncDE    => "INC DE",
            &Opcode::LdA      => "LD A, d8",
            &Opcode::XORA     => "XOR A",
            &Opcode::LdCA     => "LD C, A",
            &Opcode::PopBC    => "POP BC",
            &Opcode::PushBC   => "PUSH BC",
            &Opcode::CallNN   => "CALL NN",
            &Opcode::LdhA     => "LDH ($FF00+n), A",
            &Opcode::LdCADDA  => "LD ($FF00+C), A",
            &Opcode::LdHLA    => "LD (HL), A",
            &Opcode::LdC      => "LD C, d8",
            &Opcode::CB       => "",
            &Opcode::CB7C     => "BIT 7, H",
            &Opcode::CBRLC    => "RL C",
            &Opcode::RET      => "RET",
        };
        write!(f, "{}", code)
    }
}

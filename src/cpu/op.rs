use std::fmt;

const OPCODE_NO_OP: u8       = 0x00;
const OPCODE_LD_BC_A: u8     = 0x02;
const OPCODE_INC_BC: u8      = 0x03;
const OPCODE_INC_B: u8       = 0x04;
const OPCODE_DEC_B: u8       = 0x05;
const OPCODE_LD_B: u8        = 0x06;
const OPCODE_DEC_BC: u8      = 0x0B;
const OPCODE_INC_C: u8       = 0x0C;
const OPCODE_LD_DE: u8       = 0x11;
const OPCODE_INC_DE: u8      = 0x13;
const OPCODE_INC_D: u8       = 0x14;
const OPCODE_RLA: u8         = 0x17;
const OPCODE_LD_A_DE: u8     = 0x1A;
const OPCODE_INC_E: u8       = 0x1C;
const OPCODE_JR_NZ: u8       = 0x20;
const OPCODE_JR_Z: u8        = 0x28;
const OPCODE_LD_HL: u8       = 0x21;
const OPCODE_INC_H: u8       = 0x24;
const OPCODE_LD_SP: u8       = 0x31;
const OPCODE_LD_DE_A: u8     = 0x12;
const OPCODE_LD_HL_DEC_A: u8 = 0x32;
const OPCODE_LD_HL_ADD_A: u8 = 0x22;
const OPCODE_LD_A_HL_ADD: u8 = 0x2A;
const OPCODE_INC_HL: u8      = 0x23;
const OPCODE_LD_A: u8        = 0x3E;
const OPCODE_LD_A_B: u8      = 0x78;
const OPCODE_LD_A_E: u8      = 0x7B;
const OPCODE_LD_A_H: u8      = 0x7C;
const OPCODE_XOR_A: u8       = 0xAF;
const OPCODE_LD_C_A: u8      = 0x4F;
const OPCODE_POP_BC: u8      = 0xC1;
const OPCODE_PUSH_BC: u8     = 0xC5;
const OPCODE_CALL_NN: u8     = 0xCD;
const OPCODE_LD_C: u8        = 0x0E;
const OPCODE_LDH_A:u8        = 0xE0;
const OPCODE_LDH_A_A8: u8    = 0xF0;
const OPCODE_LD_C_ADD_A:u8   = 0xE2;
const OPCODE_LD_HL_A: u8     = 0x77;
const OPCODE_CP_D8: u8       = 0xFE;
const OPCODE_CP_HL: u8       = 0xBE;
const OPCODE_LD_A16_A: u8    = 0xEA;

const OPCODE_CB: u8          = 0xCB;
const OPCODE_CB_7C: u8       = 0x7C;
const OPCODE_CB_RLC: u8      = 0x11;
const OPCODE_DEC_A: u8       = 0x3D;
const OPCODE_DEC_C: u8       = 0x0D;
const OPCODE_DEC_E: u8       = 0x1D;
const OPCODE_DEC_D: u8       = 0x15;
const OPCODE_LD_H_A: u8      = 0x67;
const OPCODE_LD_D_A: u8      = 0x57;
const OPCODE_LD_B_A: u8      = 0x47;
const OPCODE_LD_E: u8        = 0x1E;
const OPCODE_SUB_B: u8       = 0x90;
const OPCODE_LD_D: u8        = 0x16;
const OPCODE_JR: u8          = 0x18;
const OPCODE_RET_NZ: u8      = 0xC0;
const OPCODE_JP: u8          = 0xC3;
const OPCODE_ADC_A: u8       = 0xCE;
const OPCODE_LD_H_HL: u8        = 0x66;
const OPCODE_CALL_Z:u8       = 0xCC;

const OPCODE_RET: u8         = 0xC9;

pub enum Opcode {
    NoOp,
    IncB,
    DecB,
    IncBC,
    DecBC,
    DecC,
    DecE,
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
    LdAE,
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
    CpD8,
    Lda16A,
    DecA,
    JRZ,
    LdHA,
    LdDA,
    LdE,
    LdHAA8,
    IncH,
    LdAH,
    SubB,
    DecD,
    LdD,
    Jr,
    CpHL,
    Jp,
    LdBA,
    LdAHLADD,
    LdDEA,
    IncE,
    IncD,
    LdAB,
    RetNZ,
    LdBCA,
    AdcA,
    LdHHL,
    CallZ,
}

impl Opcode {
    pub fn parse(address: u16, bits: u8) -> Opcode {
        match bits {
            OPCODE_INC_B       => Opcode::IncB,
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
            OPCODE_LD_A_E      => Opcode::LdAE,
            OPCODE_XOR_A       => Opcode::XORA,
            OPCODE_LD_C_A      => Opcode::LdCA,
            OPCODE_POP_BC      => Opcode::PopBC,
            OPCODE_PUSH_BC     => Opcode::PushBC,
            OPCODE_CALL_NN     => Opcode::CallNN,
            OPCODE_LDH_A       => Opcode::LdhA,
            OPCODE_LDH_A_A8    => Opcode::LdHAA8,
            OPCODE_LD_C_ADD_A  => Opcode::LdCADDA,
            OPCODE_LD_HL_A     => Opcode::LdHLA,
            OPCODE_LD_C        => Opcode::LdC,
            OPCODE_CB          => Opcode::CB,
            OPCODE_LD_A_DE     => Opcode::LdADE,
            OPCODE_RET         => Opcode::RET,
            OPCODE_CP_D8       => Opcode::CpD8,
            OPCODE_LD_A16_A    => Opcode::Lda16A,
            OPCODE_DEC_A       => Opcode::DecA,
            OPCODE_JR_Z        => Opcode::JRZ,
            OPCODE_LD_H_A      => Opcode::LdHA,
            OPCODE_LD_D_A      => Opcode::LdDA,
            OPCODE_LD_E        => Opcode::LdE,
            OPCODE_DEC_C       => Opcode::DecC,
            OPCODE_DEC_E       => Opcode::DecE,
            OPCODE_INC_H       => Opcode::IncH,
            OPCODE_LD_A_H      => Opcode::LdAH,
            OPCODE_SUB_B       => Opcode::SubB,
            OPCODE_DEC_D       => Opcode::DecD,
            OPCODE_LD_D        => Opcode::LdD,
            OPCODE_JR          => Opcode::Jr,
            OPCODE_CP_HL       => Opcode::CpHL,
            OPCODE_NO_OP       => Opcode::NoOp,
            OPCODE_JP          => Opcode::Jp,
            OPCODE_LD_B_A      => Opcode::LdBA,
            OPCODE_LD_A_HL_ADD => Opcode::LdAHLADD,
            OPCODE_LD_DE_A     => Opcode::LdDEA,
            OPCODE_INC_E       => Opcode::IncE,
            OPCODE_INC_D       => Opcode::IncD,
            OPCODE_LD_A_B      => Opcode::LdAB,
            OPCODE_RET_NZ      => Opcode::RetNZ,
            OPCODE_LD_BC_A     => Opcode::LdBCA,
            OPCODE_ADC_A       => Opcode::AdcA,
            OPCODE_CALL_Z      => Opcode::CallZ,
            OPCODE_DEC_BC      => Opcode::DecBC,
            OPCODE_INC_BC      => Opcode::IncBC,
            OPCODE_LD_H_HL     => Opcode::LdHHL,
            _                  => {
                println!("Unimplemented opcode PC: 0x{:04X} OP: 0x{:02X}", address, bits);
                panic!();
            }
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
            &Opcode::NoOp     => "NOOP",
            &Opcode::IncB     => "INC B",
            &Opcode::DecB     => "DEC B",
            &Opcode::DecE     => "DEC E",
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
            &Opcode::LdAE     => "LD A, E",
            &Opcode::XORA     => "XOR A",
            &Opcode::LdCA     => "LD C, A",
            &Opcode::PopBC    => "POP BC",
            &Opcode::PushBC   => "PUSH BC",
            &Opcode::CallNN   => "CALL NN",
            &Opcode::LdhA     => "LDH ($FF00+n), A",
            &Opcode::LdHAA8   => "LDH A, ($FF00+a8)",
            &Opcode::LdCADDA  => "LD ($FF00+C), A",
            &Opcode::LdHLA    => "LD (HL), A",
            &Opcode::LdC      => "LD C, d8",
            &Opcode::CB       => "",
            &Opcode::CB7C     => "BIT 7, H",
            &Opcode::CBRLC    => "RL C",
            &Opcode::RET      => "RET",
            &Opcode::CpD8     => "CP d8",
            &Opcode::Lda16A   => "LD (a16), A",
            &Opcode::DecA     => "DEC A",
            &Opcode::JRZ      => "JR Z, r8",
            &Opcode::LdHA     => "LD H, A",
            &Opcode::LdDA     => "LD D, A",
            &Opcode::LdE      => "LD E, d8",
            &Opcode::DecC     => "DEC C",
            &Opcode::IncH     => "INC H",
            &Opcode::LdAH     => "LD A, H",
            &Opcode::SubB     => "SUB B",
            &Opcode::DecD     => "DEC D",
            &Opcode::LdD      => "LD D, d8",
            &Opcode::Jr       => "JR d8",
            &Opcode::Jp       => "JP (a16)",
            &Opcode::CpHL     => "CP (HL)",
            &Opcode::LdBA     => "LD B, A",
            &Opcode::LdAHLADD => "Ld A, (HL+)",
            &Opcode::LdDEA    => "Ld (DE), A",
            &Opcode::IncE     => "INC E",
            &Opcode::IncD     => "INC D",
            &Opcode::LdAB     => "LD A, B",
            &Opcode::RetNZ    => "RET NZ",
            &Opcode::LdBCA    => "LD (BC), A",
            &Opcode::AdcA     => "ADC A, d8",
            &Opcode::LdHHL    => "LD H, (HL)",
            &Opcode::CallZ    => "CALL Z, a16",
            &Opcode::DecBC    => "DEC BC",
            &Opcode::IncBC    => "INC BC",
        };
        write!(f, "{}", code)
    }
}

use asm_riscv::{I, Reg};

pub trait Stringify{
    fn to_string(&self)->String;

    fn to_string_relative(&self, address:u32)->String;
}

impl Stringify for I{
    fn to_string(&self)->String{
        match *self{
            Self::ADD { d, s1, s2 } => format!("ADD {:?}, {:?}, {:?}", d, s1, s2),
            Self::ADDI { d, s, im } => format!("ADDI {:?}, {:?}, {}", d, s, im),
            Self::AND { d, s1, s2 } => format!("AND {:?}, {:?}, {:?}", d, s1, s2),
            Self::ANDI { d, s, im } => format!("AND {:?}, {:?}, {}", d, s, im),
            Self::AUIPC { d, im } => format!("AUIPC {:?}, {}", d, im),
            Self::BEQ { s1, s2, im } => format!("BEQ {:?}, {:?}, {}", s1, s2, im),
            Self::BGE { s1, s2, im } => format!("BGE {:?}, {:?}, {}", s1, s2, im),
            Self::BGEU { s1, s2, im } => format!("BGEU {:?}, {:?}, {}", s1, s2, im),
            Self::BLT { s1, s2, im } => format!("BLT {:?}, {:?}, {}", s1, s2, im),
            Self::BLTU { s1, s2, im } => format!("BLTU {:?}, {:?}, {}", s1, s2, im),
            Self::BNE { s1, s2, im } => format!("BNE {:?}, {:?}, {}", s1, s2, im),
            Self::JAL { d, im } => format!("JAL {:?}, {}",d,  im),
            Self::JALR { d, s, im } => format!("JALR {:?}, {:?}, {}",d, s, im),
            Self::LW { d, s, im } => format!("LW {:?}, {}({:?})", d, im, s),
            Self::SW { s1, s2, im } => format!("SW {:?}, {}({:?})", s2, im, s1),
            Self::LUI { d, im } => format!("LUI {:?}, {}", d, im),
            Self::SLLI { d, s, im } => format!("SLLI {:?}, {:?}, {}", d, s, im),
            Self::ORI { d, s, im } => format!("ORI {:?}, {:?}, {}", d, s, im),
            Self::OR { d, s1, s2 } => format!("OR {:?}, {:?}, {:?}", d, s1, s2),
            Self::SRAI { d, s, im } => format!("SRAI {:?}, {:?}, {}", d, s, im),
            Self::SRLI { d, s, im } => format!("SRLI {:?}, {:?}, {}", d, s, im),
            Self::XOR { d, s1, s2 } => format!("XOR {:?}, {:?}, {:?}", d, s1, s2),
            Self::LB { d, s, im } => format!("LB {:?}, {}({:?})", d, im, s),
            Self::LBU { d, s, im } => format!("LBU {:?}, {}({:?})", d, im, s),
            Self::LHU { d, s, im } => format!("LBU {:?}, {}({:?})", d, im, s),
            Self::SB { s1, s2, im } => format!("SB {:?}, {}({:?})", s2, im, s1),
            Self::SH { s1, s2, im } => format!("SHU {:?}, {}({:?})", s2, im, s1),
            Self::SLT { d, s1, s2 } => format!("SLT {:?}, {:?}, {:?}", d, s1, s2),
            Self::SLTI { d, s, im } => format!("SLTI {:?}, {:?}, {}", d, s, im),
            Self::SLTU { d , s1, s2 } => format!("SLTU {:?}, {:?}, {:?}", d, s1, s2),
            Self::SLTUI { d, s, im } => format!("SLTIU {:?}, {:?}, {}", d, s, im as u16),
            Self::SUB { d, s1, s2 } => format!("SUB {:?}, {:?}, {:?}", d, s1, s2),
            Self::XORI { d, s, im } => format!("XORI {:?}, {:?}, {}", d, s, im),
            Self::LH { d, s, im } => format!("LH {:?}, {}({:?})", d, im, s),
            Self::SLL { d, s1, s2 } => format!("SLL {:?}, {:?}, {:?}", d, s1, s2),
            Self::SRL { d, s1, s2 } => format!("SRL {:?}, {:?}, {:?}", d, s1, s2),
            Self::SRA { d, s1, s2 } => format!("SRA {:?}, {:?}, {:?}", d, s1, s2),
            Self::EBREAK {  }=>format!("EBREAK"),
            Self::ECALL {  }=>format!("ECALL"),
            Self::FENCE { im }=>format!("FENCE {:?}", im),
            Self::CSRRW { csr, s1, d } => format!("CSRRW {:x}, {:?}, {:?}", csr, s1, d),
            Self::CSRRS { csr, s1, d } => format!("CSRRS {:x}, {:?}, {:?}", csr, s1, d),
            Self::CSRRC { csr, s1, d } => format!("CSRRC {:x}, {:?}, {:?}", csr, s1, d),
            Self::CSRRWI { csr, zimm, d } => format!("CSRRW {:x}, {:?}, {:?}", csr, zimm, d),
            Self::CSRRSI { csr, zimm, d } => format!("CSRRSI {:x}, {:?}, {:?}", csr, zimm, d),
            Self::CSRRCI { csr, zimm, d } => format!("CSRRCI {:x}, {:?}, {:?}", csr, zimm, d),
            Self::MRET {} => format!("MRET"),


            //_=>{"todo".to_string()}
        }
    }
    fn to_string_relative(&self, address:u32)->String{
        "todo".to_string()
    }
}

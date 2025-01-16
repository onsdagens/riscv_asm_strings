pub use asm_riscv::Reg;
pub use asm_riscv::I;
pub trait Stringify {
    fn to_string(&self) -> String;

    fn to_string_relative(&self, address: u32) -> String;
}

impl Stringify for I {
    fn to_string(&self) -> String {
        match *self {
            Self::ADD { d, s1, s2 } => format!("ADD {}, {}, {}", d, s1, s2),
            Self::ADDI { d, s, im } => {
                format!("ADDI {}, {}, {}", d, s, (((im & 0xFFF) as i32) << 20) >> 20)
            }
            Self::AND { d, s1, s2 } => format!("AND {}, {}, {}", d, s1, s2),
            Self::ANDI { d, s, im } => {
                format!("AND {}, {}, {}", d, s, (((im & 0xFFF) as i32) << 20) >> 20)
            }
            Self::AUIPC { d, im } => format!("AUIPC {}, {}", d, im),
            Self::BEQ { s1, s2, im } => format!(
                "BEQ {}, {}, {}",
                s1,
                s2,
                (((im & 0xFFF) as i32) << 20) >> 20
            ),
            Self::BGE { s1, s2, im } => format!(
                "BGE {}, {}, {}",
                s1,
                s2,
                (((im & 0xFFF) as i32) << 20) >> 20
            ),
            Self::BGEU { s1, s2, im } => format!(
                "BGEU {}, {}, {}",
                s1,
                s2,
                (((im & 0xFFF) as i32) << 20) >> 20
            ),
            Self::BLT { s1, s2, im } => format!(
                "BLT {}, {}, {}",
                s1,
                s2,
                (((im & 0xFFF) as i32) << 20) >> 20
            ),
            Self::BLTU { s1, s2, im } => format!(
                "BLTU {}, {}, {}",
                s1,
                s2,
                (((im & 0xFFF) as i32) << 20) >> 20
            ),
            Self::BNE { s1, s2, im } => format!(
                "BNE {}, {}, {}",
                s1,
                s2,
                (((im & 0xFFF) as i32) << 20) >> 20
            ),
            Self::JAL { d, im } => format!("JAL {}, {}", d, im),
            Self::JALR { d, s, im } => format!("JALR {}, {}, {}", d, s, im),
            Self::LW { d, s, im } => {
                format!("LW {}, {}({})", d, (((im & 0xFFF) as i32) << 20) >> 20, s)
            }
            Self::SW { s1, s2, im } => {
                format!("SW {}, {}({})", s2, (((im & 0xFFF) as i32) << 20) >> 20, s1)
            }
            Self::LUI { d, im } => format!("LUI {}, {}", d, im),
            Self::SLLI { d, s, im } => format!("SLLI {}, {}, {}", d, s, im),
            Self::ORI { d, s, im } => {
                format!("ORI {}, {}, {}", d, s, (((im & 0xFFF) as i32) << 20) >> 20)
            }
            Self::OR { d, s1, s2 } => format!("OR {}, {}, {}", d, s1, s2),
            Self::SRAI { d, s, im } => format!("SRAI {}, {}, {}", d, s, im),
            Self::SRLI { d, s, im } => format!("SRLI {}, {}, {}", d, s, im),
            Self::XOR { d, s1, s2 } => format!("XOR {}, {}, {}", d, s1, s2),
            Self::LB { d, s, im } => {
                format!("LB {}, {}({})", d, (((im & 0xFFF) as i32) << 20) >> 20, s)
            }
            Self::LBU { d, s, im } => {
                format!("LBU {}, {}({})", d, (((im & 0xFFF) as i32) << 20) >> 20, s)
            }
            Self::LHU { d, s, im } => {
                format!("LBU {}, {}({})", d, (((im & 0xFFF) as i32) << 20) >> 20, s)
            }
            Self::SB { s1, s2, im } => {
                format!("SB {}, {}({})", s2, (((im & 0xFFF) as i32) << 20) >> 20, s1)
            }
            Self::SH { s1, s2, im } => format!(
                "SHU {}, {}({})",
                s2,
                (((im & 0xFFF) as i32) << 20) >> 20,
                s1
            ),
            Self::SLT { d, s1, s2 } => format!("SLT {}, {}, {}", d, s1, s2),
            Self::SLTI { d, s, im } => {
                format!("SLTI {}, {}, {}", d, s, (((im & 0xFFF) as i32) << 20) >> 20)
            }
            Self::SLTU { d, s1, s2 } => format!("SLTU {}, {}, {}", d, s1, s2),
            Self::SLTUI { d, s, im } => format!("SLTIU {}, {}, {}", d, s, im as u16),
            Self::SUB { d, s1, s2 } => format!("SUB {}, {}, {}", d, s1, s2),
            Self::XORI { d, s, im } => {
                format!("XORI {}, {}, {}", d, s, (((im & 0xFFF) as i32) << 20) >> 20)
            }
            Self::LH { d, s, im } => {
                format!("LH {}, {}({})", d, (((im & 0xFFF) as i32) << 20) >> 20, s)
            }
            Self::SLL { d, s1, s2 } => format!("SLL {}, {}, {}", d, s1, s2),
            Self::SRL { d, s1, s2 } => format!("SRL {}, {}, {}", d, s1, s2),
            Self::SRA { d, s1, s2 } => format!("SRA {}, {}, {}", d, s1, s2),
            Self::EBREAK {} => "EBREAK".to_string(),
            Self::ECALL {} => "ECALL".to_string(),
            Self::FENCE { im } => format!("FENCE {}", im),
            Self::CSRRW { csr, s1, d } => format!("CSRRW 0x{:x}, {}, {}", csr, d, s1),
            Self::CSRRS { csr, s1, d } => format!("CSRRS {:x}, {}, {}", csr, s1, d),
            Self::CSRRC { csr, s1, d } => format!("CSRRC {:x}, {}, {}", csr, s1, d),
            Self::CSRRWI { csr, zimm, d } => format!("CSRRW {:x}, {}, {}", csr, zimm, d),
            Self::CSRRSI { csr, zimm, d } => format!("CSRRSI {:x}, {}, {}", csr, zimm, d),
            Self::CSRRCI { csr, zimm, d } => format!("CSRRCI {:x}, {}, {}", csr, zimm, d),
            Self::MRET {} => "MRET".to_string(),
            //_=>{"todo".to_string()}
        }
    }
    fn to_string_relative(&self, _address: u32) -> String {
        "todo".to_string()
    }
}

pub trait StringifyLowerHex {
    fn to_string(&self) -> String;
}

impl StringifyLowerHex for I {
    fn to_string(&self) -> String {
        match *self {
            Self::ADD { d, s1, s2 } => format!("ADD {}, {}, {}", d, s1, s2),
            Self::ADDI { d, s, im } => format!("ADDI {}, {}, {:#x}", d, s, im & 0xFFF),
            Self::AND { d, s1, s2 } => format!("AND {}, {}, {}", d, s1, s2),
            Self::ANDI { d, s, im } => format!("AND {}, {}, {:#x}", d, s, im & 0xFFF),
            Self::AUIPC { d, im } => format!("AUIPC {}, {:#x}", d, im),
            Self::BEQ { s1, s2, im } => format!("BEQ {}, {}, {:#x}", s1, s2, im & 0xFFF),
            Self::BGE { s1, s2, im } => format!("BGE {}, {}, {:#x}", s1, s2, im & 0xFFF),
            Self::BGEU { s1, s2, im } => format!("BGEU {}, {}, {:#x}", s1, s2, im & 0xFFF),
            Self::BLT { s1, s2, im } => format!("BLT {}, {}, {:#x}", s1, s2, im & 0xFFF),
            Self::BLTU { s1, s2, im } => format!("BLTU {}, {}, {:#x}", s1, s2, im & 0xFFF),
            Self::BNE { s1, s2, im } => format!("BNE {}, {}, {:#x}", s1, s2, im & 0xFFF),
            Self::JAL { d, im } => format!("JAL {}, {:#x}", d, im),
            Self::JALR { d, s, im } => format!("JALR {}, {}, {:#x}", d, s, im),
            Self::LW { d, s, im } => format!("LW {}, {:#x}({})", d, im & 0xFFF, s),
            Self::SW { s1, s2, im } => format!("SW {}, {:#x}({})", s2, im & 0xFFF, s1),
            Self::LUI { d, im } => format!("LUI {}, {:#x}", d, im),
            Self::SLLI { d, s, im } => format!("SLLI {}, {}, {:#x}", d, s, im),
            Self::ORI { d, s, im } => format!("ORI {}, {}, {:#x}", d, s, im & 0xFFF),
            Self::OR { d, s1, s2 } => format!("OR {}, {}, {}", d, s1, s2),
            Self::SRAI { d, s, im } => format!("SRAI {}, {}, {:#x}", d, s, im),
            Self::SRLI { d, s, im } => format!("SRLI {}, {}, {:#x}", d, s, im),
            Self::XOR { d, s1, s2 } => format!("XOR {}, {}, {}", d, s1, s2),
            Self::LB { d, s, im } => format!("LB {}, {:#x}({})", d, im & 0xFFF, s),
            Self::LBU { d, s, im } => format!("LBU {}, {:#x}({})", d, im & 0xFFF, s),
            Self::LHU { d, s, im } => format!("LBU {}, {:#x}({})", d, im & 0xFFF, s),
            Self::SB { s1, s2, im } => format!("SB {}, {:#x}({})", s2, im & 0xFFF, s1),
            Self::SH { s1, s2, im } => format!("SHU {}, {:#x}({})", s2, im & 0xFFF, s1),
            Self::SLT { d, s1, s2 } => format!("SLT {}, {}, {}", d, s1, s2),
            Self::SLTI { d, s, im } => format!("SLTI {}, {}, {:#x}", d, s, im & 0xFFF),
            Self::SLTU { d, s1, s2 } => format!("SLTU {}, {}, {}", d, s1, s2),
            Self::SLTUI { d, s, im } => format!("SLTIU {}, {}, {:#x}", d, s, im as u16 & 0xFFF),
            Self::SUB { d, s1, s2 } => format!("SUB {}, {}, {}", d, s1, s2),
            Self::XORI { d, s, im } => format!("XORI {}, {}, {:#x}", d, s, im & 0xFFF),
            Self::LH { d, s, im } => format!("LH {}, {:#x}({})", d, im & 0xFFF, s),
            Self::SLL { d, s1, s2 } => format!("SLL {}, {}, {}", d, s1, s2),
            Self::SRL { d, s1, s2 } => format!("SRL {}, {}, {}", d, s1, s2),
            Self::SRA { d, s1, s2 } => format!("SRA {}, {}, {}", d, s1, s2),
            Self::EBREAK {} => "EBREAK".to_string(),
            Self::ECALL {} => "ECALL".to_string(),
            Self::FENCE { im } => format!("FENCE {:#x}", im),
            Self::CSRRW { csr, s1, d } => format!("CSRRW {:x}, {}, {}", csr, s1, d),
            Self::CSRRS { csr, s1, d } => format!("CSRRS {:x}, {}, {}", csr, s1, d),
            Self::CSRRC { csr, s1, d } => format!("CSRRC {:x}, {}, {}", csr, s1, d),
            Self::CSRRWI { csr, zimm, d } => format!("CSRRW {:x}, {:#x}, {}", csr, zimm, d),
            Self::CSRRSI { csr, zimm, d } => format!("CSRRSI {:x}, {:#x}, {}", csr, zimm, d),
            Self::CSRRCI { csr, zimm, d } => format!("CSRRCI {:x}, {:#x}, {}", csr, zimm, d),
            Self::MRET {} => "MRET".to_string(),
        }
    }
}

pub trait StringifyUpperHex {
    fn to_string(&self) -> String;
}

impl StringifyUpperHex for I {
    fn to_string(&self) -> String {
        match *self {
            Self::ADD { d, s1, s2 } => format!("ADD {}, {}, {}", d, s1, s2),
            Self::ADDI { d, s, im } => format!("ADDI {}, {}, {:#X}", d, s, im),
            Self::AND { d, s1, s2 } => format!("AND {}, {}, {}", d, s1, s2),
            Self::ANDI { d, s, im } => format!("AND {}, {}, {:#X}", d, s, im),
            Self::AUIPC { d, im } => format!("AUIPC {}, {:#X}", d, im),
            Self::BEQ { s1, s2, im } => format!("BEQ {}, {}, {:#X}", s1, s2, im),
            Self::BGE { s1, s2, im } => format!("BGE {}, {}, {:#X}", s1, s2, im),
            Self::BGEU { s1, s2, im } => format!("BGEU {}, {}, {:#X}", s1, s2, im),
            Self::BLT { s1, s2, im } => format!("BLT {}, {}, {:#X}", s1, s2, im),
            Self::BLTU { s1, s2, im } => format!("BLTU {}, {}, {:#X}", s1, s2, im),
            Self::BNE { s1, s2, im } => format!("BNE {}, {}, {:#X}", s1, s2, im),
            Self::JAL { d, im } => format!("JAL {}, {:#X}", d, im),
            Self::JALR { d, s, im } => format!("JALR {}, {}, {:#X}", d, s, im),
            Self::LW { d, s, im } => format!("LW {}, {:#X}({})", d, im, s),
            Self::SW { s1, s2, im } => format!("SW {}, {:#X}({})", s2, im, s1),
            Self::LUI { d, im } => format!("LUI {}, {:#X}", d, im),
            Self::SLLI { d, s, im } => format!("SLLI {}, {}, {:#X}", d, s, im),
            Self::ORI { d, s, im } => format!("ORI {}, {}, {:#X}", d, s, im),
            Self::OR { d, s1, s2 } => format!("OR {}, {}, {}", d, s1, s2),
            Self::SRAI { d, s, im } => format!("SRAI {}, {}, {:#X}", d, s, im),
            Self::SRLI { d, s, im } => format!("SRLI {}, {}, {:#X}", d, s, im),
            Self::XOR { d, s1, s2 } => format!("XOR {}, {}, {}", d, s1, s2),
            Self::LB { d, s, im } => format!("LB {}, {:#X}({})", d, im, s),
            Self::LBU { d, s, im } => format!("LBU {}, {:#X}({})", d, im, s),
            Self::LHU { d, s, im } => format!("LBU {}, {:#X}({})", d, im, s),
            Self::SB { s1, s2, im } => format!("SB {}, {:#X}({})", s2, im, s1),
            Self::SH { s1, s2, im } => format!("SHU {}, {:#X}({})", s2, im, s1),
            Self::SLT { d, s1, s2 } => format!("SLT {}, {}, {}", d, s1, s2),
            Self::SLTI { d, s, im } => format!("SLTI {}, {}, {:#X}", d, s, im),
            Self::SLTU { d, s1, s2 } => format!("SLTU {}, {}, {}", d, s1, s2),
            Self::SLTUI { d, s, im } => format!("SLTIU {}, {}, {:#X}", d, s, im as u16),
            Self::SUB { d, s1, s2 } => format!("SUB {}, {}, {}", d, s1, s2),
            Self::XORI { d, s, im } => format!("XORI {}, {}, {:#X}", d, s, im),
            Self::LH { d, s, im } => format!("LH {}, {:#X}({})", d, im, s),
            Self::SLL { d, s1, s2 } => format!("SLL {}, {}, {}", d, s1, s2),
            Self::SRL { d, s1, s2 } => format!("SRL {}, {}, {}", d, s1, s2),
            Self::SRA { d, s1, s2 } => format!("SRA {}, {}, {}", d, s1, s2),
            Self::EBREAK {} => "EBREAK".to_string(),
            Self::ECALL {} => "ECALL".to_string(),
            Self::FENCE { im } => format!("FENCE {:#X}", im),
            Self::CSRRW { csr, s1, d } => format!("CSRRW {:x}, {}, {}", csr, s1, d),
            Self::CSRRS { csr, s1, d } => format!("CSRRS {:x}, {}, {}", csr, s1, d),
            Self::CSRRC { csr, s1, d } => format!("CSRRC {:x}, {}, {}", csr, s1, d),
            Self::CSRRWI { csr, zimm, d } => format!("CSRRW {:x}, {:#X}, {}", csr, zimm, d),
            Self::CSRRSI { csr, zimm, d } => format!("CSRRSI {:x}, {:#X}, {}", csr, zimm, d),
            Self::CSRRCI { csr, zimm, d } => format!("CSRRCI {:x}, {:#X}, {}", csr, zimm, d),
            Self::MRET {} => "MRET".to_string(),
        }
    }
}

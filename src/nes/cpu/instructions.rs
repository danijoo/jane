use failure::err_msg;
use phf::{Map,phf_map};
use failure::{Error};
use std::fmt;
use std::fmt::{Debug,Display};

#[derive(Debug,PartialEq)]
pub enum AddrMode {
    IMP, // Implied
    IMM, // Immediate
    ZP0, // Zero page
    ZPX, // Zero Page with X (ZPX and ZPY are the same at nesdev) 
    ZPY, // Zero Page with Y (ZPX and ZPY are the same at nesdev) 
    REL, // Relatvive (Only for branching)
    ABS, // Absolute address
    ABX, // Absolute with X offset
    ABY, // Absolute with Y offset
    IND, // Indirect addressing
    IZX, // Pre Indexed 
    IZY, // Post Indexed
}

impl fmt::Display for AddrMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug,PartialEq)]
pub enum Operation {
    ADC,
    AHX,
    ALR,
    ANC,
    AND,
    ARR,
    ASL,
    AXS,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DCP,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    ISB,
    JMP,
    JSR,
    KIL,
    LAS,
    LAX,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    RLA,
    ROL,
    ROR,
    RRA,
    RTI,
    RTS,
    SAX,
    SBC,
    SEC,
    SED,
    SEI,
    SHX,
    SHY,
    SLO,
    SRE,
    STA,
    STX,
    STY,
    TAS,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    XAA,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

static INSTRUCTION_SET: Map<u8, Instruction> = phf_map! {
    // 0x00
    0x00u8 => Instruction { opcode: 0x00, addr_mode: AddrMode::IMP, operation: Operation::BRK, cycles: [7, 0] }, 
    0x01u8 => Instruction { opcode: 0x01, addr_mode: AddrMode::IZX, operation: Operation::ORA, cycles: [6, 0] }, 
    0x02u8 => Instruction { opcode: 0x02, addr_mode: AddrMode::IMP, operation: Operation::KIL, cycles: [1, 0] },
    0x03u8 => Instruction { opcode: 0x03, addr_mode: AddrMode::IZX, operation: Operation::SLO, cycles: [8, 0] }, 
    0x04u8 => Instruction { opcode: 0x04, addr_mode: AddrMode::ZP0, operation: Operation::NOP, cycles: [3, 0] },
    0x05u8 => Instruction { opcode: 0x05, addr_mode: AddrMode::ZP0, operation: Operation::ORA, cycles: [3, 0] }, 
    0x06u8 => Instruction { opcode: 0x06, addr_mode: AddrMode::ZP0, operation: Operation::ASL, cycles: [5, 0] }, 
    0x07u8 => Instruction { opcode: 0x07, addr_mode: AddrMode::ZP0, operation: Operation::SLO, cycles: [5, 0] }, 
    0x08u8 => Instruction { opcode: 0x08, addr_mode: AddrMode::IMP, operation: Operation::PHP, cycles: [3, 0] }, 
    0x09u8 => Instruction { opcode: 0x09, addr_mode: AddrMode::IMM, operation: Operation::ORA, cycles: [2, 0] }, 
    0x0au8 => Instruction { opcode: 0x0a, addr_mode: AddrMode::IMP, operation: Operation::ASL, cycles: [2, 0] },
    0x0bu8 => Instruction { opcode: 0x0b, addr_mode: AddrMode::IMM, operation: Operation::ANC, cycles: [2, 0] },
    0x0cu8 => Instruction { opcode: 0x0c, addr_mode: AddrMode::ABS, operation: Operation::NOP, cycles: [4, 0] },
    0x0du8 => Instruction { opcode: 0x0d, addr_mode: AddrMode::ABS, operation: Operation::ORA, cycles: [4, 0] }, 
    0x0eu8 => Instruction { opcode: 0x0e, addr_mode: AddrMode::ABS, operation: Operation::ASL, cycles: [6, 0] }, 
    0x0fu8 => Instruction { opcode: 0x0f, addr_mode: AddrMode::ABS, operation: Operation::SLO, cycles: [6, 0] }, 
    // 0x10
    0x10u8 => Instruction { opcode: 0x10, addr_mode: AddrMode::REL, operation: Operation::BPL, cycles: [2, 1] }, 
    0x11u8 => Instruction { opcode: 0x11, addr_mode: AddrMode::IZY, operation: Operation::ORA, cycles: [5, 1] }, 
    0x12u8 => Instruction { opcode: 0x12, addr_mode: AddrMode::IMP, operation: Operation::KIL, cycles: [1, 0] },
    0x13u8 => Instruction { opcode: 0x13, addr_mode: AddrMode::IZY, operation: Operation::SLO, cycles: [8, 0] }, 
    0x14u8 => Instruction { opcode: 0x14, addr_mode: AddrMode::ZPX, operation: Operation::NOP, cycles: [4, 0] },
    0x15u8 => Instruction { opcode: 0x15, addr_mode: AddrMode::ZPX, operation: Operation::ORA, cycles: [4, 0] }, 
    0x16u8 => Instruction { opcode: 0x16, addr_mode: AddrMode::ZPX, operation: Operation::ASL, cycles: [6, 0] }, 
    0x17u8 => Instruction { opcode: 0x17, addr_mode: AddrMode::ZPX, operation: Operation::SLO, cycles: [6, 0] }, 
    0x18u8 => Instruction { opcode: 0x18, addr_mode: AddrMode::IMP, operation: Operation::CLC, cycles: [2, 0] }, 
    0x19u8 => Instruction { opcode: 0x19, addr_mode: AddrMode::ABY, operation: Operation::ORA, cycles: [4, 1] },
    0x1au8 => Instruction { opcode: 0x1a, addr_mode: AddrMode::IMP, operation: Operation::NOP, cycles: [2, 0] },
    0x1bu8 => Instruction { opcode: 0x1b, addr_mode: AddrMode::ABY, operation: Operation::SLO, cycles: [7, 0] }, 
    0x1cu8 => Instruction { opcode: 0x1c, addr_mode: AddrMode::ABX, operation: Operation::NOP, cycles: [4, 1] },
    0x1du8 => Instruction { opcode: 0x1d, addr_mode: AddrMode::ABX, operation: Operation::ORA, cycles: [4, 1] }, 
    0x1eu8 => Instruction { opcode: 0x1e, addr_mode: AddrMode::ABX, operation: Operation::ASL, cycles: [7, 0] }, 
    0x1fu8 => Instruction { opcode: 0x1f, addr_mode: AddrMode::ABX, operation: Operation::SLO, cycles: [7, 0] }, 
    // 0x20
    0x20u8 => Instruction { opcode: 0x20, addr_mode: AddrMode::ABS, operation: Operation::JSR, cycles: [6, 0] }, 
    0x21u8 => Instruction { opcode: 0x21, addr_mode: AddrMode::IZX, operation: Operation::AND, cycles: [6, 0] }, 
    0x22u8 => Instruction { opcode: 0x22, addr_mode: AddrMode::IMP, operation: Operation::KIL, cycles: [1, 0] },
    0x23u8 => Instruction { opcode: 0x23, addr_mode: AddrMode::IZX, operation: Operation::RLA, cycles: [8, 0] }, 
    0x24u8 => Instruction { opcode: 0x24, addr_mode: AddrMode::ZP0, operation: Operation::BIT, cycles: [3, 0] }, 
    0x25u8 => Instruction { opcode: 0x25, addr_mode: AddrMode::ZP0, operation: Operation::AND, cycles: [3, 0] }, 
    0x26u8 => Instruction { opcode: 0x26, addr_mode: AddrMode::ZP0, operation: Operation::ROL, cycles: [5, 0] }, 
    0x27u8 => Instruction { opcode: 0x27, addr_mode: AddrMode::ZP0, operation: Operation::RLA, cycles: [5, 0] }, 
    0x28u8 => Instruction { opcode: 0x28, addr_mode: AddrMode::IMP, operation: Operation::PLP, cycles: [4, 0] }, 
    0x29u8 => Instruction { opcode: 0x29, addr_mode: AddrMode::IMM, operation: Operation::AND, cycles: [2, 0] }, 
    0x2au8 => Instruction { opcode: 0x2a, addr_mode: AddrMode::IMP, operation: Operation::ROL, cycles: [2, 0] }, 
    0x2bu8 => Instruction { opcode: 0x2b, addr_mode: AddrMode::IMM, operation: Operation::ANC, cycles: [2, 0] }, 
    0x2cu8 => Instruction { opcode: 0x2c, addr_mode: AddrMode::ABS, operation: Operation::BIT, cycles: [4, 0] }, 
    0x2du8 => Instruction { opcode: 0x2d, addr_mode: AddrMode::ABS, operation: Operation::AND, cycles: [4, 0] }, 
    0x2eu8 => Instruction { opcode: 0x2e, addr_mode: AddrMode::ABS, operation: Operation::ROL, cycles: [6, 0] }, 
    0x2fu8 => Instruction { opcode: 0x2f, addr_mode: AddrMode::ABS, operation: Operation::RLA, cycles: [6, 0] }, 
    // 0x30
    0x30u8 => Instruction { opcode: 0x30, addr_mode: AddrMode::REL, operation: Operation::BMI, cycles: [2, 1] }, 
    0x31u8 => Instruction { opcode: 0x31, addr_mode: AddrMode::IZY, operation: Operation::AND, cycles: [5, 1] }, 
    0x32u8 => Instruction { opcode: 0x32, addr_mode: AddrMode::IMP, operation: Operation::KIL, cycles: [1, 0] },
    0x33u8 => Instruction { opcode: 0x33, addr_mode: AddrMode::IZY, operation: Operation::RLA, cycles: [8, 0] },
    0x34u8 => Instruction { opcode: 0x34, addr_mode: AddrMode::ZPX, operation: Operation::NOP, cycles: [4, 0] },
    0x35u8 => Instruction { opcode: 0x35, addr_mode: AddrMode::ZPX, operation: Operation::AND, cycles: [4, 0] }, 
    0x36u8 => Instruction { opcode: 0x36, addr_mode: AddrMode::ZPX, operation: Operation::ROL, cycles: [6, 0] },
    0x37u8 => Instruction { opcode: 0x37, addr_mode: AddrMode::ZPX, operation: Operation::RLA, cycles: [6, 0] },
    0x38u8 => Instruction { opcode: 0x38, addr_mode: AddrMode::IMP, operation: Operation::SEC, cycles: [2, 0] }, 
    0x39u8 => Instruction { opcode: 0x39, addr_mode: AddrMode::ABY, operation: Operation::AND, cycles: [4, 1] },
    0x3au8 => Instruction { opcode: 0x3a, addr_mode: AddrMode::IMP, operation: Operation::NOP, cycles: [2, 0] },
    0x3bu8 => Instruction { opcode: 0x3b, addr_mode: AddrMode::ABY, operation: Operation::RLA, cycles: [7, 0] },
    0x3cu8 => Instruction { opcode: 0x3c, addr_mode: AddrMode::ABX, operation: Operation::NOP, cycles: [4, 1] },
    0x3du8 => Instruction { opcode: 0x3d, addr_mode: AddrMode::ABX, operation: Operation::AND, cycles: [4, 1] }, 
    0x3eu8 => Instruction { opcode: 0x3e, addr_mode: AddrMode::ABX, operation: Operation::ROL, cycles: [7, 0] }, 
    0x3fu8 => Instruction { opcode: 0x3f, addr_mode: AddrMode::ABX, operation: Operation::RLA, cycles: [7, 0] }, 
    // 0x40
    0x40u8 => Instruction { opcode: 0x40, addr_mode: AddrMode::IMP, operation: Operation::RTI, cycles: [6, 0] }, 
    0x41u8 => Instruction { opcode: 0x41, addr_mode: AddrMode::IZX, operation: Operation::EOR, cycles: [6, 0] },  
    0x42u8 => Instruction { opcode: 0x42, addr_mode: AddrMode::IMP, operation: Operation::KIL, cycles: [1, 0] },
    0x43u8 => Instruction { opcode: 0x43, addr_mode: AddrMode::IZX, operation: Operation::SRE, cycles: [8, 0] }, 
    0x44u8 => Instruction { opcode: 0x44, addr_mode: AddrMode::ZP0, operation: Operation::NOP, cycles: [3, 0] },
    0x45u8 => Instruction { opcode: 0x45, addr_mode: AddrMode::ZP0, operation: Operation::EOR, cycles: [3, 0] }, 
    0x46u8 => Instruction { opcode: 0x46, addr_mode: AddrMode::ZP0, operation: Operation::LSR, cycles: [5, 0] }, 
    0x47u8 => Instruction { opcode: 0x47, addr_mode: AddrMode::ZP0, operation: Operation::SRE, cycles: [5, 0] }, 
    0x48u8 => Instruction { opcode: 0x48, addr_mode: AddrMode::IMP, operation: Operation::PHA, cycles: [3, 0] }, 
    0x49u8 => Instruction { opcode: 0x49, addr_mode: AddrMode::IMM, operation: Operation::EOR, cycles: [2, 0] }, 
    0x4au8 => Instruction { opcode: 0x4a, addr_mode: AddrMode::IMP, operation: Operation::LSR, cycles: [2, 0] }, 
    0x4bu8 => Instruction { opcode: 0x4b, addr_mode: AddrMode::IMM, operation: Operation::ALR, cycles: [2, 0] }, 
    0x4cu8 => Instruction { opcode: 0x4c, addr_mode: AddrMode::ABS, operation: Operation::JMP, cycles: [3, 0] }, 
    0x4du8 => Instruction { opcode: 0x4d, addr_mode: AddrMode::ABS, operation: Operation::EOR, cycles: [4, 0] }, 
    0x4eu8 => Instruction { opcode: 0x4e, addr_mode: AddrMode::ABS, operation: Operation::LSR, cycles: [6, 0] }, 
    0x4fu8 => Instruction { opcode: 0x4f, addr_mode: AddrMode::ABS, operation: Operation::SRE, cycles: [6, 0] }, 
    // 0x50
    0x50u8 => Instruction { opcode: 0x50, addr_mode: AddrMode::REL, operation: Operation::BVC, cycles: [2, 1] }, 
    0x51u8 => Instruction { opcode: 0x51, addr_mode: AddrMode::IZY, operation: Operation::EOR, cycles: [5, 1] }, 
    0x52u8 => Instruction { opcode: 0x52, addr_mode: AddrMode::IMP, operation: Operation::KIL, cycles: [1, 0] }, 
    0x53u8 => Instruction { opcode: 0x53, addr_mode: AddrMode::IZY, operation: Operation::SRE, cycles: [8, 0] }, 
    0x54u8 => Instruction { opcode: 0x54, addr_mode: AddrMode::ZPX, operation: Operation::NOP, cycles: [4, 0] },
    0x55u8 => Instruction { opcode: 0x55, addr_mode: AddrMode::ZPX, operation: Operation::EOR, cycles: [4, 0] }, 
    0x56u8 => Instruction { opcode: 0x56, addr_mode: AddrMode::ZPX, operation: Operation::LSR, cycles: [6, 0] }, 
    0x57u8 => Instruction { opcode: 0x57, addr_mode: AddrMode::ZPX, operation: Operation::SRE, cycles: [6, 0] }, 
    0x58u8 => Instruction { opcode: 0x58, addr_mode: AddrMode::IMP, operation: Operation::CLI, cycles: [2, 0] }, 
    0x59u8 => Instruction { opcode: 0x59, addr_mode: AddrMode::ABY, operation: Operation::EOR, cycles: [4, 1] }, 
    0x5au8 => Instruction { opcode: 0x5a, addr_mode: AddrMode::IMP, operation: Operation::NOP, cycles: [2, 0] },
    0x5bu8 => Instruction { opcode: 0x5b, addr_mode: AddrMode::ABY, operation: Operation::SRE, cycles: [7, 0] },
    0x5cu8 => Instruction { opcode: 0x5c, addr_mode: AddrMode::ABX, operation: Operation::NOP, cycles: [4, 1] },
    0x5du8 => Instruction { opcode: 0x5d, addr_mode: AddrMode::ABX, operation: Operation::EOR, cycles: [4, 1] }, 
    0x5eu8 => Instruction { opcode: 0x5e, addr_mode: AddrMode::ABX, operation: Operation::LSR, cycles: [7, 0] }, 
    0x5fu8 => Instruction { opcode: 0x5f, addr_mode: AddrMode::ABX, operation: Operation::SRE, cycles: [7, 0] }, 
    // 0x60
    0x60u8 => Instruction { opcode: 0x60, addr_mode: AddrMode::IMP, operation: Operation::RTS, cycles: [6, 0] }, 
    0x61u8 => Instruction { opcode: 0x61, addr_mode: AddrMode::IZX, operation: Operation::ADC, cycles: [6, 0] }, 
    0x62u8 => Instruction { opcode: 0x62, addr_mode: AddrMode::IMP, operation: Operation::KIL, cycles: [1, 0] },
    0x63u8 => Instruction { opcode: 0x63, addr_mode: AddrMode::IZX, operation: Operation::RRA, cycles: [8, 0] }, 
    0x64u8 => Instruction { opcode: 0x64, addr_mode: AddrMode::ZP0, operation: Operation::NOP, cycles: [3, 0] },
    0x65u8 => Instruction { opcode: 0x65, addr_mode: AddrMode::ZP0, operation: Operation::ADC, cycles: [3, 0] }, 
    0x66u8 => Instruction { opcode: 0x66, addr_mode: AddrMode::ZP0, operation: Operation::ROR, cycles: [5, 0] },
    0x67u8 => Instruction { opcode: 0x67, addr_mode: AddrMode::ZP0, operation: Operation::RRA, cycles: [5, 0] }, 
    0x68u8 => Instruction { opcode: 0x68, addr_mode: AddrMode::IMP, operation: Operation::PLA, cycles: [4, 0] }, 
    0x69u8 => Instruction { opcode: 0x69, addr_mode: AddrMode::IMM, operation: Operation::ADC, cycles: [2, 0] }, 
    0x6au8 => Instruction { opcode: 0x6a, addr_mode: AddrMode::IMP, operation: Operation::ROR, cycles: [2, 0] }, 
    0x6bu8 => Instruction { opcode: 0x6b, addr_mode: AddrMode::IMM, operation: Operation::ARR, cycles: [2, 0] }, 
    0x6cu8 => Instruction { opcode: 0x6c, addr_mode: AddrMode::IND, operation: Operation::JMP, cycles: [5, 0] }, 
    0x6du8 => Instruction { opcode: 0x6d, addr_mode: AddrMode::ABS, operation: Operation::ADC, cycles: [4, 0] }, 
    0x6eu8 => Instruction { opcode: 0x6e, addr_mode: AddrMode::ABS, operation: Operation::ROR, cycles: [6, 0] }, 
    0x6fu8 => Instruction { opcode: 0x6f, addr_mode: AddrMode::ABS, operation: Operation::RRA, cycles: [6, 0] }, 
    // 0x70
    0x70u8 => Instruction { opcode: 0x70, addr_mode: AddrMode::REL, operation: Operation::BVS, cycles: [2, 1] }, 
    0x71u8 => Instruction { opcode: 0x71, addr_mode: AddrMode::IZY, operation: Operation::ADC, cycles: [5, 1] },
    0x72u8 => Instruction { opcode: 0x72, addr_mode: AddrMode::IMP, operation: Operation::KIL, cycles: [1, 0] }, 
    0x73u8 => Instruction { opcode: 0x73, addr_mode: AddrMode::IZY, operation: Operation::RRA, cycles: [8, 0] }, 
    0x74u8 => Instruction { opcode: 0x74, addr_mode: AddrMode::ZPX, operation: Operation::NOP, cycles: [4, 0] },
    0x75u8 => Instruction { opcode: 0x75, addr_mode: AddrMode::ZPX, operation: Operation::ADC, cycles: [4, 0] }, 
    0x76u8 => Instruction { opcode: 0x76, addr_mode: AddrMode::ZPX, operation: Operation::ROR, cycles: [6, 0] }, 
    0x77u8 => Instruction { opcode: 0x77, addr_mode: AddrMode::ZPX, operation: Operation::RRA, cycles: [6, 0] }, 
    0x78u8 => Instruction { opcode: 0x78, addr_mode: AddrMode::IMP, operation: Operation::SEI, cycles: [2, 0] }, 
    0x79u8 => Instruction { opcode: 0x79, addr_mode: AddrMode::ABY, operation: Operation::ADC, cycles: [4, 1] }, 
    0x7au8 => Instruction { opcode: 0x7a, addr_mode: AddrMode::IMP, operation: Operation::NOP, cycles: [2, 0] },
    0x7bu8 => Instruction { opcode: 0x7b, addr_mode: AddrMode::ABY, operation: Operation::RRA, cycles: [7, 0] },
    0x7cu8 => Instruction { opcode: 0x7c, addr_mode: AddrMode::ABX, operation: Operation::NOP, cycles: [4, 1] },
    0x7du8 => Instruction { opcode: 0x7d, addr_mode: AddrMode::ABX, operation: Operation::ADC, cycles: [4, 1] }, 
    0x7eu8 => Instruction { opcode: 0x7e, addr_mode: AddrMode::ABX, operation: Operation::ROR, cycles: [7, 0] }, 
    0x7fu8 => Instruction { opcode: 0x7f, addr_mode: AddrMode::ABX, operation: Operation::RRA, cycles: [7, 0] },
    // 0x80
    0x80u8 => Instruction { opcode: 0x80, addr_mode: AddrMode::IMM, operation: Operation::NOP, cycles: [2, 0] },
    0x81u8 => Instruction { opcode: 0x81, addr_mode: AddrMode::IZX, operation: Operation::STA, cycles: [6, 0] }, 
    0x82u8 => Instruction { opcode: 0x82, addr_mode: AddrMode::IMM, operation: Operation::NOP, cycles: [2, 0] },
    0x83u8 => Instruction { opcode: 0x83, addr_mode: AddrMode::IZX, operation: Operation::SAX, cycles: [6, 0] },
    0x84u8 => Instruction { opcode: 0x84, addr_mode: AddrMode::ZP0, operation: Operation::STY, cycles: [3, 0] }, 
    0x85u8 => Instruction { opcode: 0x85, addr_mode: AddrMode::ZP0, operation: Operation::STA, cycles: [3, 0] }, 
    0x86u8 => Instruction { opcode: 0x86, addr_mode: AddrMode::ZP0, operation: Operation::STX, cycles: [3, 0] }, 
    0x87u8 => Instruction { opcode: 0x87, addr_mode: AddrMode::ZP0, operation: Operation::SAX, cycles: [3, 0] },
    0x88u8 => Instruction { opcode: 0x88, addr_mode: AddrMode::IMP, operation: Operation::DEY, cycles: [2, 0] }, 
    0x89u8 => Instruction { opcode: 0x89, addr_mode: AddrMode::IMM, operation: Operation::NOP, cycles: [2, 0] },
    0x8au8 => Instruction { opcode: 0x8a, addr_mode: AddrMode::IMP, operation: Operation::TXA, cycles: [2, 0] }, 
    0x8bu8 => Instruction { opcode: 0x8b, addr_mode: AddrMode::IMM, operation: Operation::XAA, cycles: [2, 0] }, 
    0x8cu8 => Instruction { opcode: 0x8c, addr_mode: AddrMode::ABS, operation: Operation::STY, cycles: [4, 0] }, 
    0x8du8 => Instruction { opcode: 0x8d, addr_mode: AddrMode::ABS, operation: Operation::STA, cycles: [4, 0] }, 
    0x8eu8 => Instruction { opcode: 0x8e, addr_mode: AddrMode::ABS, operation: Operation::STX, cycles: [4, 0] }, 
    0x8fu8 => Instruction { opcode: 0x8f, addr_mode: AddrMode::ABS, operation: Operation::SAX, cycles: [4, 0] },
    // 0x90
    0x90u8 => Instruction { opcode: 0x90, addr_mode: AddrMode::REL, operation: Operation::BCC, cycles: [2, 1] }, 
    0x91u8 => Instruction { opcode: 0x91, addr_mode: AddrMode::IZY, operation: Operation::STA, cycles: [6, 0] },
    0x92u8 => Instruction { opcode: 0x92, addr_mode: AddrMode::IMP, operation: Operation::KIL, cycles: [1, 0] },
    0x93u8 => Instruction { opcode: 0x93, addr_mode: AddrMode::IZY, operation: Operation::AHX, cycles: [6, 0] }, 
    0x94u8 => Instruction { opcode: 0x94, addr_mode: AddrMode::ZPX, operation: Operation::STY, cycles: [4, 0] }, 
    0x95u8 => Instruction { opcode: 0x95, addr_mode: AddrMode::ZPX, operation: Operation::STA, cycles: [4, 0] }, 
    0x96u8 => Instruction { opcode: 0x96, addr_mode: AddrMode::ZPY, operation: Operation::STX, cycles: [4, 0] }, 
    0x97u8 => Instruction { opcode: 0x97, addr_mode: AddrMode::ZPY, operation: Operation::SAX, cycles: [4, 0] },
    0x98u8 => Instruction { opcode: 0x98, addr_mode: AddrMode::IMP, operation: Operation::TYA, cycles: [2, 0] }, 
    0x99u8 => Instruction { opcode: 0x99, addr_mode: AddrMode::ABY, operation: Operation::STA, cycles: [5, 0] }, 
    0x9au8 => Instruction { opcode: 0x9a, addr_mode: AddrMode::IMP, operation: Operation::TXS, cycles: [2, 0] }, 
    0x9bu8 => Instruction { opcode: 0x9b, addr_mode: AddrMode::ABY, operation: Operation::TAS, cycles: [5, 0] }, 
    0x9cu8 => Instruction { opcode: 0x9c, addr_mode: AddrMode::ABX, operation: Operation::SHY, cycles: [5, 0] }, 
    0x9du8 => Instruction { opcode: 0x9d, addr_mode: AddrMode::ABX, operation: Operation::STA, cycles: [5, 0] }, 
    0x9eu8 => Instruction { opcode: 0x9e, addr_mode: AddrMode::ABY, operation: Operation::SHX, cycles: [5, 0] }, 
    0x9fu8 => Instruction { opcode: 0x9f, addr_mode: AddrMode::ABY, operation: Operation::AHX, cycles: [5, 0] }, 
    // 0xa0
    0xa0u8 => Instruction { opcode: 0xa0, addr_mode: AddrMode::IMM, operation: Operation::LDY, cycles: [2, 0] }, 
    0xa1u8 => Instruction { opcode: 0xa1, addr_mode: AddrMode::IZX, operation: Operation::LDA, cycles: [6, 0] }, 
    0xa2u8 => Instruction { opcode: 0xa2, addr_mode: AddrMode::IMM, operation: Operation::LDX, cycles: [2, 0] }, 
    0xa3u8 => Instruction { opcode: 0xa3, addr_mode: AddrMode::IZX, operation: Operation::LAX, cycles: [6, 0] },
    0xa4u8 => Instruction { opcode: 0xa4, addr_mode: AddrMode::ZP0, operation: Operation::LDY, cycles: [3, 0] }, 
    0xa5u8 => Instruction { opcode: 0xa5, addr_mode: AddrMode::ZP0, operation: Operation::LDA, cycles: [3, 0] }, 
    0xa6u8 => Instruction { opcode: 0xa6, addr_mode: AddrMode::ZP0, operation: Operation::LDX, cycles: [3, 0] }, 
    0xa7u8 => Instruction { opcode: 0xa7, addr_mode: AddrMode::ZP0, operation: Operation::LAX, cycles: [3, 0] },
    0xa8u8 => Instruction { opcode: 0xa8, addr_mode: AddrMode::IMP, operation: Operation::TAY, cycles: [2, 0] }, 
    0xa9u8 => Instruction { opcode: 0xa9, addr_mode: AddrMode::IMM, operation: Operation::LDA, cycles: [2, 0] }, 
    0xaau8 => Instruction { opcode: 0xaa, addr_mode: AddrMode::IMP, operation: Operation::TAX, cycles: [2, 0] }, 
    0xabu8 => Instruction { opcode: 0xab, addr_mode: AddrMode::IMM, operation: Operation::LAX, cycles: [2, 0] }, 
    0xacu8 => Instruction { opcode: 0xac, addr_mode: AddrMode::ABS, operation: Operation::LDY, cycles: [4, 0] }, 
    0xadu8 => Instruction { opcode: 0xad, addr_mode: AddrMode::ABS, operation: Operation::LDA, cycles: [4, 0] }, 
    0xaeu8 => Instruction { opcode: 0xae, addr_mode: AddrMode::ABS, operation: Operation::LDX, cycles: [4, 0] }, 
    0xafu8 => Instruction { opcode: 0xaf, addr_mode: AddrMode::ABS, operation: Operation::LAX, cycles: [4, 0] }, 
    // 0xb0
    0xb0u8 => Instruction { opcode: 0xb0, addr_mode: AddrMode::REL, operation: Operation::BCS, cycles: [2, 1] }, 
    0xb1u8 => Instruction { opcode: 0xb1, addr_mode: AddrMode::IZY, operation: Operation::LDA, cycles: [5, 1] }, 
    0xb2u8 => Instruction { opcode: 0xb2, addr_mode: AddrMode::IMP, operation: Operation::KIL, cycles: [1, 0] }, 
    0xb3u8 => Instruction { opcode: 0xb3, addr_mode: AddrMode::IZY, operation: Operation::LAX, cycles: [5, 1] },
    0xb4u8 => Instruction { opcode: 0xb4, addr_mode: AddrMode::ZPX, operation: Operation::LDY, cycles: [4, 0] }, 
    0xb5u8 => Instruction { opcode: 0xb5, addr_mode: AddrMode::ZPX, operation: Operation::LDA, cycles: [4, 0] }, 
    0xb6u8 => Instruction { opcode: 0xb6, addr_mode: AddrMode::ZPY, operation: Operation::LDX, cycles: [4, 0] }, 
    0xb7u8 => Instruction { opcode: 0xb7, addr_mode: AddrMode::ZPY, operation: Operation::LAX, cycles: [4, 0] },
    0xb8u8 => Instruction { opcode: 0xb8, addr_mode: AddrMode::IMP, operation: Operation::CLV, cycles: [2, 0] }, 
    0xb9u8 => Instruction { opcode: 0xb9, addr_mode: AddrMode::ABY, operation: Operation::LDA, cycles: [4, 1] }, 
    0xbau8 => Instruction { opcode: 0xba, addr_mode: AddrMode::IMP, operation: Operation::TSX, cycles: [2, 0] }, 
    0xbbu8 => Instruction { opcode: 0xbb, addr_mode: AddrMode::ABY, operation: Operation::LAS, cycles: [4, 1] }, 
    0xbcu8 => Instruction { opcode: 0xbc, addr_mode: AddrMode::ABX, operation: Operation::LDY, cycles: [4, 1] }, 
    0xbdu8 => Instruction { opcode: 0xbd, addr_mode: AddrMode::ABX, operation: Operation::LDA, cycles: [4, 1] }, 
    0xbeu8 => Instruction { opcode: 0xbe, addr_mode: AddrMode::ABY, operation: Operation::LDX, cycles: [4, 1] }, 
    0xbfu8 => Instruction { opcode: 0xbf, addr_mode: AddrMode::ABY, operation: Operation::LAX, cycles: [4, 1] },
    // 0xc0
    0xc0u8 => Instruction { opcode: 0xc0, addr_mode: AddrMode::IMM, operation: Operation::CPY, cycles: [2, 0] }, 
    0xc1u8 => Instruction { opcode: 0xc1, addr_mode: AddrMode::IZX, operation: Operation::CMP, cycles: [6, 0] }, 
    0xc2u8 => Instruction { opcode: 0xc2, addr_mode: AddrMode::IMM, operation: Operation::NOP, cycles: [2, 0] },
    0xc3u8 => Instruction { opcode: 0xc3, addr_mode: AddrMode::IZX, operation: Operation::DCP, cycles: [8, 0] }, 
    0xc4u8 => Instruction { opcode: 0xc4, addr_mode: AddrMode::ZP0, operation: Operation::CPY, cycles: [3, 0] }, 
    0xc5u8 => Instruction { opcode: 0xc5, addr_mode: AddrMode::ZP0, operation: Operation::CMP, cycles: [3, 0] }, 
    0xc6u8 => Instruction { opcode: 0xc6, addr_mode: AddrMode::ZP0, operation: Operation::DEC, cycles: [5, 0] }, 
    0xc7u8 => Instruction { opcode: 0xc7, addr_mode: AddrMode::ZP0, operation: Operation::DCP, cycles: [5, 0] }, 
    0xc8u8 => Instruction { opcode: 0xc8, addr_mode: AddrMode::IMP, operation: Operation::INY, cycles: [2, 0] }, 
    0xc9u8 => Instruction { opcode: 0xc9, addr_mode: AddrMode::IMM, operation: Operation::CMP, cycles: [2, 0] }, 
    0xcau8 => Instruction { opcode: 0xca, addr_mode: AddrMode::IMP, operation: Operation::DEX, cycles: [2, 0] }, 
    0xcbu8 => Instruction { opcode: 0xcb, addr_mode: AddrMode::IMM, operation: Operation::AXS, cycles: [1, 0] }, 
    0xccu8 => Instruction { opcode: 0xcc, addr_mode: AddrMode::ABS, operation: Operation::CPY, cycles: [4, 0] }, 
    0xcdu8 => Instruction { opcode: 0xcd, addr_mode: AddrMode::ABS, operation: Operation::CMP, cycles: [4, 0] }, 
    0xceu8 => Instruction { opcode: 0xce, addr_mode: AddrMode::ABS, operation: Operation::DEC, cycles: [6, 0] }, 
    0xcfu8 => Instruction { opcode: 0xcf, addr_mode: AddrMode::ABS, operation: Operation::DCP, cycles: [6, 0] }, 
    // 0xd0
    0xd0u8 => Instruction { opcode: 0xd0, addr_mode: AddrMode::REL, operation: Operation::BNE, cycles: [2, 1] }, 
    0xd1u8 => Instruction { opcode: 0xd1, addr_mode: AddrMode::IZY, operation: Operation::CMP, cycles: [5, 1] }, 
    0xd2u8 => Instruction { opcode: 0xd2, addr_mode: AddrMode::IMP, operation: Operation::KIL, cycles: [1, 0] }, 
    0xd3u8 => Instruction { opcode: 0xd3, addr_mode: AddrMode::IZY, operation: Operation::DCP, cycles: [8, 0] }, 
    0xd4u8 => Instruction { opcode: 0xd4, addr_mode: AddrMode::ZPX, operation: Operation::NOP, cycles: [4, 0] },
    0xd5u8 => Instruction { opcode: 0xd5, addr_mode: AddrMode::ZPX, operation: Operation::CMP, cycles: [4, 0] }, 
    0xd6u8 => Instruction { opcode: 0xd6, addr_mode: AddrMode::ZPX, operation: Operation::DEC, cycles: [6, 0] }, 
    0xd7u8 => Instruction { opcode: 0xd7, addr_mode: AddrMode::ZPX, operation: Operation::DCP, cycles: [6, 1] }, 
    0xd8u8 => Instruction { opcode: 0xd8, addr_mode: AddrMode::IMP, operation: Operation::CLD, cycles: [2, 0] }, 
    0xd9u8 => Instruction { opcode: 0xd9, addr_mode: AddrMode::ABY, operation: Operation::CMP, cycles: [4, 0] }, 
    0xdau8 => Instruction { opcode: 0xda, addr_mode: AddrMode::IMP, operation: Operation::NOP, cycles: [2, 0] },
    0xdbu8 => Instruction { opcode: 0xdb, addr_mode: AddrMode::ABY, operation: Operation::DCP, cycles: [7, 0] }, 
    0xdcu8 => Instruction { opcode: 0xdc, addr_mode: AddrMode::ABX, operation: Operation::NOP, cycles: [4, 1] },
    0xddu8 => Instruction { opcode: 0xdd, addr_mode: AddrMode::ABX, operation: Operation::CMP, cycles: [4, 1] }, 
    0xdeu8 => Instruction { opcode: 0xde, addr_mode: AddrMode::ABX, operation: Operation::DEC, cycles: [7, 0] }, 
    0xdfu8 => Instruction { opcode: 0xdf, addr_mode: AddrMode::ABX, operation: Operation::DCP, cycles: [7, 0] }, 
    // 0xe0
    0xe0u8 => Instruction { opcode: 0xe0, addr_mode: AddrMode::IMM, operation: Operation::CPX, cycles: [2, 0] }, 
    0xe1u8 => Instruction { opcode: 0xe1, addr_mode: AddrMode::IZX, operation: Operation::SBC, cycles: [6, 0] }, 
    0xe2u8 => Instruction { opcode: 0xe2, addr_mode: AddrMode::IMM, operation: Operation::NOP, cycles: [2, 0] },
    0xe3u8 => Instruction { opcode: 0xe3, addr_mode: AddrMode::IZX, operation: Operation::ISB, cycles: [8, 0] }, 
    0xe4u8 => Instruction { opcode: 0xe4, addr_mode: AddrMode::ZP0, operation: Operation::CPX, cycles: [3, 0] }, 
    0xe5u8 => Instruction { opcode: 0xe5, addr_mode: AddrMode::ZP0, operation: Operation::SBC, cycles: [3, 0] }, 
    0xe6u8 => Instruction { opcode: 0xe6, addr_mode: AddrMode::ZP0, operation: Operation::INC, cycles: [5, 0] }, 
    0xe7u8 => Instruction { opcode: 0xe7, addr_mode: AddrMode::ZP0, operation: Operation::ISB, cycles: [5, 0] }, 
    0xe8u8 => Instruction { opcode: 0xe8, addr_mode: AddrMode::IMP, operation: Operation::INX, cycles: [2, 0] }, 
    0xe9u8 => Instruction { opcode: 0xe9, addr_mode: AddrMode::IMM, operation: Operation::SBC, cycles: [2, 0] }, 
    0xeau8 => Instruction { opcode: 0xea, addr_mode: AddrMode::IMP, operation: Operation::NOP, cycles: [2, 0] }, 
    0xebu8 => Instruction { opcode: 0xeb, addr_mode: AddrMode::IMM, operation: Operation::SBC, cycles: [2, 0] }, 
    0xecu8 => Instruction { opcode: 0xec, addr_mode: AddrMode::ABS, operation: Operation::CPX, cycles: [4, 0] }, 
    0xedu8 => Instruction { opcode: 0xed, addr_mode: AddrMode::ABS, operation: Operation::SBC, cycles: [4, 0] }, 
    0xeeu8 => Instruction { opcode: 0xee, addr_mode: AddrMode::ABS, operation: Operation::INC, cycles: [6, 0] }, 
    0xefu8 => Instruction { opcode: 0xef, addr_mode: AddrMode::ABS, operation: Operation::ISB, cycles: [6, 0] }, 
    // 0xf0
    0xf0u8 => Instruction { opcode: 0xf0, addr_mode: AddrMode::REL, operation: Operation::BEQ, cycles: [2, 1] }, 
    0xf1u8 => Instruction { opcode: 0xf1, addr_mode: AddrMode::IZY, operation: Operation::SBC, cycles: [5, 1] }, 
    0xf2u8 => Instruction { opcode: 0xf2, addr_mode: AddrMode::IMP, operation: Operation::KIL, cycles: [1, 0] }, 
    0xf3u8 => Instruction { opcode: 0xf3, addr_mode: AddrMode::IZY, operation: Operation::ISB, cycles: [8, 0] }, 
    0xf4u8 => Instruction { opcode: 0xf4, addr_mode: AddrMode::ZPX, operation: Operation::NOP, cycles: [4, 0] },
    0xf5u8 => Instruction { opcode: 0xf5, addr_mode: AddrMode::ZPX, operation: Operation::SBC, cycles: [4, 0] }, 
    0xf6u8 => Instruction { opcode: 0xf6, addr_mode: AddrMode::ZPX, operation: Operation::INC, cycles: [6, 0] }, 
    0xf7u8 => Instruction { opcode: 0xf7, addr_mode: AddrMode::ZPX, operation: Operation::ISB, cycles: [6, 0] }, 
    0xf8u8 => Instruction { opcode: 0xf8, addr_mode: AddrMode::IMP, operation: Operation::SED, cycles: [2, 0] }, 
    0xf9u8 => Instruction { opcode: 0xf9, addr_mode: AddrMode::ABY, operation: Operation::SBC, cycles: [4, 1] }, 
    0xfau8 => Instruction { opcode: 0xfa, addr_mode: AddrMode::IMP, operation: Operation::NOP, cycles: [2, 0] },
    0xfbu8 => Instruction { opcode: 0xfb, addr_mode: AddrMode::ABY, operation: Operation::ISB, cycles: [7, 0] }, 
    0xfcu8 => Instruction { opcode: 0xfc, addr_mode: AddrMode::ABX, operation: Operation::NOP, cycles: [4, 1] },
    0xfdu8 => Instruction { opcode: 0xfd, addr_mode: AddrMode::ABX, operation: Operation::SBC, cycles: [4, 1] }, 
    0xfeu8 => Instruction { opcode: 0xfe, addr_mode: AddrMode::ABX, operation: Operation::INC, cycles: [7, 0] }, 
    0xffu8 => Instruction { opcode: 0xff, addr_mode: AddrMode::ABX, operation: Operation::ISB, cycles: [7, 0] }, 
};

pub struct Instruction {
    pub opcode: u8,
    pub addr_mode: AddrMode,
    pub operation: Operation,

    // number of cycles required
    // first value: number of cycles
    // second value: additional cycles on page cross
    pub cycles: [u8; 2],  
}

impl Instruction {
    pub fn decode_op(opcode: u8) -> &'static Instruction {
        INSTRUCTION_SET.get(&opcode)
            .expect(&format!("Unknown opcode: {:#04x}", opcode))
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.cycles[1] == 0 {
            write!(f, "Instruction {{ opcode: {:#x}, op/addr: {:?}/{:?}, cycles: {:?} }}",
                self.opcode, self.operation, self.addr_mode, self.cycles[0])
        } else {
            write!(f, "Instruction {{ opcode: {:#x}, op/addr: {:?}/{:?}, cycles: {:?}(+{:?}) }}",
                self.opcode, self.operation, self.addr_mode, self.cycles[0],self.cycles[1])
        }
        
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_opcode_matches_inst_opcode() {
        for opcode in 0x00..0xFF {
            let inst = Instruction::decode_op(opcode);
            assert_eq!(opcode, inst.opcode,
                "opcode not matching for {:#04x}", opcode);
        }
    }

    #[test]
    fn test_kill_addr_mode() {
        for opcode in 0x00..0xFF {
            let inst = Instruction::decode_op(opcode);
            if inst.operation == Operation::KIL {
                assert_eq!(inst.addr_mode, AddrMode::IMP,
                    "KIL addr mode not IMP for {:#04x}", opcode);
                assert_eq!(inst.cycles, [1, 0],
                    "KIL cycles not 0 for {:#04x}", opcode);
            }
        }
    }

       
}

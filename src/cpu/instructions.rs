#[derive(Copy, Clone)]
pub enum Instruction {
    ADD(ArithmeticType),
    ADC,
    SUB(ArithmeticByteTarget),
    SBC,
    AND(ArithmeticByteTarget),
    OR(ArithmeticByteTarget),
    XOR(ArithmeticByteTarget),
    CP(ArithmeticByteTarget),
    INC(ArithmeticType),
    DEC(ArithmeticType),
    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RRLA,
    CPL,
    BIT(u8),
    RES(u8),
    SET(u8),
    SRL,
    RR,
    RL,
    RRC,
    RLC,
    SRA,
    SLA,
    SWAP,
    LD(LoadType, LoadOperation),
    JP(JumpCondition, JumpTarget),
    JR(JumpCondition),
    CALL(JumpCondition),
    RET(JumpCondition),
    PUSH(ArithmeticWordTarget),
    POP(ArithmeticWordTarget),
    RST(u16),
    EI,
    DI,
    RETI,
    PREFIX,
    NOP,
    UNDEFINED,
}

#[derive(Copy, Clone)]
pub enum ArithmeticType {
    Byte(ArithmeticByteTarget),
    Word(ArithmeticWordTarget),
}

#[derive(Copy, Clone)]
pub enum ArithmeticByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    MHL,
    N8,
}

#[derive(Copy, Clone)]
pub enum ArithmeticWordTarget {
    HL,
    BC,
    DE,
    AF,
}

#[derive(Copy, Clone)]
pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    MHL,
    MBC,
    MDE,
    MN16,
    DN8,
    DC,
}

#[derive(Copy, Clone)]
pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    N8,
    DN8,
    DC,
    MHL,
    MDE,
    MN16,
}

#[derive(Copy, Clone)]
pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Copy, Clone)]
pub enum LoadWordSource {
    N16,
}

#[derive(Copy, Clone)]
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget, LoadWordSource),
}

#[derive(Copy, Clone)]
pub enum LoadOperation {
    HLI,
    HLD,
    None,
}

#[derive(Copy, Clone)]
pub enum JumpCondition {
    Z,
    NZ,
    C,
    NC,
    NONE,
}

#[derive(Copy, Clone)]
pub enum JumpTarget {
    N16,
    HL,
}

pub const OPCODES: [(Instruction, u8); 256] = [
    // 0X
    (Instruction::NOP, 4),
    (
        Instruction::LD(
            LoadType::Word(LoadWordTarget::BC, LoadWordSource::N16),
            LoadOperation::None,
        ),
        12,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MBC, LoadByteSource::A),
            LoadOperation::None,
        ),
        8,
    ),
    (Instruction::UNDEFINED, 8),
    (
        Instruction::INC(ArithmeticType::Byte(ArithmeticByteTarget::B)),
        4,
    ),
    (
        Instruction::DEC(ArithmeticType::Byte(ArithmeticByteTarget::B)),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::B, LoadByteSource::N8),
            LoadOperation::None,
        ),
        8,
    ),
    (Instruction::UNDEFINED, 4),
    (Instruction::UNDEFINED, 20),
    (
        Instruction::ADD(ArithmeticType::Word(ArithmeticWordTarget::BC)),
        8,
    ),
    (Instruction::UNDEFINED, 8),
    (
        Instruction::DEC(ArithmeticType::Word(ArithmeticWordTarget::BC)),
        8,
    ),
    (
        Instruction::INC(ArithmeticType::Byte(ArithmeticByteTarget::C)),
        4,
    ),
    (
        Instruction::DEC(ArithmeticType::Byte(ArithmeticByteTarget::C)),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::C, LoadByteSource::N8),
            LoadOperation::None,
        ),
        8,
    ),
    (Instruction::UNDEFINED, 4),
    // 1X
    (Instruction::UNDEFINED, 4),
    (
        Instruction::LD(
            LoadType::Word(LoadWordTarget::DE, LoadWordSource::N16),
            LoadOperation::None,
        ),
        12,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MDE, LoadByteSource::A),
            LoadOperation::None,
        ),
        8,
    ),
    (
        Instruction::INC(ArithmeticType::Word(ArithmeticWordTarget::DE)),
        8,
    ),
    (Instruction::UNDEFINED, 4),
    (
        Instruction::DEC(ArithmeticType::Byte(ArithmeticByteTarget::D)),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::D, LoadByteSource::N8),
            LoadOperation::None,
        ),
        8,
    ),
    (Instruction::RLA, 4),
    (Instruction::JR(JumpCondition::NONE), 12),
    (
        Instruction::ADD(ArithmeticType::Word(ArithmeticWordTarget::DE)),
        8,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::MDE),
            LoadOperation::None,
        ),
        8,
    ),
    (Instruction::UNDEFINED, 8),
    (
        Instruction::INC(ArithmeticType::Byte(ArithmeticByteTarget::E)),
        4,
    ),
    (
        Instruction::DEC(ArithmeticType::Byte(ArithmeticByteTarget::E)),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::E, LoadByteSource::N8),
            LoadOperation::None,
        ),
        8,
    ),
    (Instruction::UNDEFINED, 4),
    // 2X
    (Instruction::JR(JumpCondition::NZ), 2),
    (
        Instruction::LD(
            LoadType::Word(LoadWordTarget::HL, LoadWordSource::N16),
            LoadOperation::None,
        ),
        12,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MHL, LoadByteSource::A),
            LoadOperation::HLI,
        ),
        8,
    ),
    (
        Instruction::INC(ArithmeticType::Word(ArithmeticWordTarget::HL)),
        8,
    ),
    (
        Instruction::INC(ArithmeticType::Byte(ArithmeticByteTarget::H)),
        4,
    ),
    (Instruction::UNDEFINED, 4),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::H, LoadByteSource::N8),
            LoadOperation::None,
        ),
        8,
    ),
    (Instruction::UNDEFINED, 4),
    (Instruction::JR(JumpCondition::Z), 2),
    (Instruction::UNDEFINED, 8),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::MHL),
            LoadOperation::HLI,
        ),
        8,
    ),
    (Instruction::UNDEFINED, 8),
    (Instruction::UNDEFINED, 4),
    (Instruction::UNDEFINED, 4),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::L, LoadByteSource::N8),
            LoadOperation::None,
        ),
        8,
    ),
    (Instruction::CPL, 4),
    // 3X
    (Instruction::UNDEFINED, 2),
    (
        Instruction::LD(
            LoadType::Word(LoadWordTarget::SP, LoadWordSource::N16),
            LoadOperation::None,
        ),
        12,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MHL, LoadByteSource::A),
            LoadOperation::HLD,
        ),
        8,
    ),
    (Instruction::UNDEFINED, 8),
    (
        Instruction::INC(ArithmeticType::Byte(ArithmeticByteTarget::MHL)),
        12,
    ),
    (Instruction::UNDEFINED, 12),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MHL, LoadByteSource::N8),
            LoadOperation::None,
        ),
        12,
    ),
    (Instruction::UNDEFINED, 4),
    (Instruction::UNDEFINED, 2),
    (Instruction::UNDEFINED, 8),
    (Instruction::UNDEFINED, 8),
    (Instruction::UNDEFINED, 8),
    (
        Instruction::INC(ArithmeticType::Byte(ArithmeticByteTarget::A)),
        4,
    ),
    (
        Instruction::DEC(ArithmeticType::Byte(ArithmeticByteTarget::A)),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::N8),
            LoadOperation::None,
        ),
        8,
    ),
    (Instruction::UNDEFINED, 4),
    // 4X
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::B, LoadByteSource::A),
            LoadOperation::None,
        ),
        0,
    ),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::C, LoadByteSource::A),
            LoadOperation::None,
        ),
        0,
    ),
    // 5X
    (Instruction::UNDEFINED, 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::D, LoadByteSource::C),
            LoadOperation::None,
        ),
        0,
    ),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::D, LoadByteSource::MHL),
            LoadOperation::None,
        ),
        0,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::D, LoadByteSource::A),
            LoadOperation::None,
        ),
        0,
    ),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::E, LoadByteSource::MHL),
            LoadOperation::None,
        ),
        0,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::E, LoadByteSource::A),
            LoadOperation::None,
        ),
        0,
    ),
    // 6X
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::H, LoadByteSource::D),
            LoadOperation::None,
        ),
        0,
    ),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::H, LoadByteSource::MHL),
            LoadOperation::None,
        ),
        0,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::H, LoadByteSource::A),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::L, LoadByteSource::B),
            LoadOperation::None,
        ),
        0,
    ),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::L, LoadByteSource::E),
            LoadOperation::None,
        ),
        0,
    ),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::L, LoadByteSource::A),
            LoadOperation::None,
        ),
        0,
    ),
    // 7X
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MHL, LoadByteSource::A),
            LoadOperation::None,
        ),
        0,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::B),
            LoadOperation::None,
        ),
        0,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::C),
            LoadOperation::None,
        ),
        0,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::D),
            LoadOperation::None,
        ),
        0,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::E),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::H),
            LoadOperation::None,
        ),
        0,
    ),
    (Instruction::UNDEFINED, 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::MHL),
            LoadOperation::None,
        ),
        0,
    ),
    (Instruction::UNDEFINED, 0),
    // 8X
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (
        Instruction::ADD(ArithmeticType::Byte(ArithmeticByteTarget::A)),
        0,
    ),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    // 9X
    (Instruction::SUB(ArithmeticByteTarget::B), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    // AX
    (Instruction::UNDEFINED, 0),
    (Instruction::AND(ArithmeticByteTarget::C), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::AND(ArithmeticByteTarget::E), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::AND(ArithmeticByteTarget::A), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::XOR(ArithmeticByteTarget::C), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::XOR(ArithmeticByteTarget::E), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::XOR(ArithmeticByteTarget::A), 0),
    // BX
    (Instruction::OR(ArithmeticByteTarget::B), 0),
    (Instruction::OR(ArithmeticByteTarget::C), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::OR(ArithmeticByteTarget::MHL), 0),
    (Instruction::OR(ArithmeticByteTarget::A), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    // CX
    (Instruction::RET(JumpCondition::NZ), 0),
    (Instruction::POP(ArithmeticWordTarget::BC), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::JP(JumpCondition::NONE, JumpTarget::N16), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::PUSH(ArithmeticWordTarget::BC), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::RET(JumpCondition::Z), 0),
    (Instruction::RET(JumpCondition::NONE), 0),
    (Instruction::JP(JumpCondition::Z, JumpTarget::N16), 0),
    (Instruction::PREFIX, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::CALL(JumpCondition::NONE), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    // DX
    (Instruction::UNDEFINED, 0),
    (Instruction::POP(ArithmeticWordTarget::DE), 0),
    (Instruction::JP(JumpCondition::NC, JumpTarget::N16), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::PUSH(ArithmeticWordTarget::DE), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::RETI, 16),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    // EX
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::DN8, LoadByteSource::A),
            LoadOperation::None,
        ),
        0,
    ),
    (Instruction::POP(ArithmeticWordTarget::HL), 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::DC, LoadByteSource::A),
            LoadOperation::None,
        ),
        0,
    ),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::PUSH(ArithmeticWordTarget::HL), 0),
    (Instruction::AND(ArithmeticByteTarget::N8), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::JP(JumpCondition::NONE, JumpTarget::HL), 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MN16, LoadByteSource::A),
            LoadOperation::None,
        ),
        0,
    ),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::RST(0x28), 0),
    // FX
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::DN8),
            LoadOperation::None,
        ),
        0,
    ),
    (Instruction::POP(ArithmeticWordTarget::AF), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::DI, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::PUSH(ArithmeticWordTarget::AF), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::MN16),
            LoadOperation::None,
        ),
        0,
    ),
    (Instruction::EI, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::CP(ArithmeticByteTarget::N8), 0),
    (Instruction::RST(0x38), 16),
];

pub const PREFIX_CODES: [(Instruction, u8); 32] = [
    (Instruction::RLC, 0),
    (Instruction::RRC, 0),
    (Instruction::RL, 0),
    (Instruction::RR, 0),
    (Instruction::SLA, 0),
    (Instruction::SRA, 0),
    (Instruction::SWAP, 0),
    (Instruction::SRL, 0),
    (Instruction::BIT(0), 0),
    (Instruction::BIT(1), 0),
    (Instruction::BIT(2), 0),
    (Instruction::BIT(3), 0),
    (Instruction::BIT(4), 0),
    (Instruction::BIT(5), 0),
    (Instruction::BIT(6), 0),
    (Instruction::BIT(7), 0),
    (Instruction::RES(0), 0),
    (Instruction::RES(1), 0),
    (Instruction::RES(2), 0),
    (Instruction::RES(3), 0),
    (Instruction::RES(4), 0),
    (Instruction::RES(5), 0),
    (Instruction::RES(6), 0),
    (Instruction::RES(7), 0),
    (Instruction::SET(0), 0),
    (Instruction::SET(1), 0),
    (Instruction::SET(2), 0),
    (Instruction::SET(3), 0),
    (Instruction::SET(4), 0),
    (Instruction::SET(5), 0),
    (Instruction::SET(6), 0),
    (Instruction::SET(7), 0),
];

pub const PREFIX_TARGETS: [ArithmeticByteTarget; 8] = [
    ArithmeticByteTarget::B,
    ArithmeticByteTarget::C,
    ArithmeticByteTarget::D,
    ArithmeticByteTarget::E,
    ArithmeticByteTarget::H,
    ArithmeticByteTarget::L,
    ArithmeticByteTarget::MHL,
    ArithmeticByteTarget::A,
];

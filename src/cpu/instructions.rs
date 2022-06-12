#[derive(Copy, Clone)]
pub enum Instruction {
    ADD(ArithmeticType),
    ADC(ArithmeticByteTarget),
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
    RLCA,
    LD(LoadType, LoadOperation),
    JP(JumpCondition, JumpTarget),
    JR(JumpCondition),
    CALL(JumpCondition),
    RET(JumpCondition),
    PUSH(ArithmeticWordTarget),
    POP(ArithmeticWordTarget),
    RST(u16),
    DAA,
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
    MBC,
    MDE,
    MN16,
}

#[derive(Copy, Clone)]
pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
    MN16,
}

#[derive(Copy, Clone)]
pub enum LoadWordSource {
    N16,
    SP,
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
    (
        Instruction::INC(ArithmeticType::Word(ArithmeticWordTarget::BC)),
        8,
    ),
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
    (Instruction::RLCA, 4),
    (
        Instruction::LD(
            LoadType::Word(LoadWordTarget::MN16, LoadWordSource::SP),
            LoadOperation::None,
        ),
        20,
    ),
    (
        Instruction::ADD(ArithmeticType::Word(ArithmeticWordTarget::BC)),
        8,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::MBC),
            LoadOperation::None,
        ),
        8,
    ),
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
    (Instruction::DAA, 4),
    (Instruction::JR(JumpCondition::Z), 2),
    (Instruction::UNDEFINED, 8),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::MHL),
            LoadOperation::HLI,
        ),
        8,
    ),
    (
        Instruction::DEC(ArithmeticType::Word(ArithmeticWordTarget::HL)),
        8,
    ),
    (
        Instruction::INC(ArithmeticType::Byte(ArithmeticByteTarget::L)),
        4,
    ),
    (
        Instruction::DEC(ArithmeticType::Byte(ArithmeticByteTarget::L)),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::L, LoadByteSource::N8),
            LoadOperation::None,
        ),
        8,
    ),
    (Instruction::CPL, 4),
    // 3X
    (Instruction::JR(JumpCondition::NC), 2),
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
    (
        Instruction::DEC(ArithmeticType::Byte(ArithmeticByteTarget::MHL)),
        12,
    ),
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
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::MHL),
            LoadOperation::HLD,
        ),
        8,
    ),
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
    (Instruction::CCF, 4),
    // 4X
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::B, LoadByteSource::B),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::B, LoadByteSource::C),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::B, LoadByteSource::D),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::B, LoadByteSource::E),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::B, LoadByteSource::H),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::B, LoadByteSource::L),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::B, LoadByteSource::MHL),
            LoadOperation::None,
        ),
        8,
    ),
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
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::C, LoadByteSource::MHL),
            LoadOperation::None,
        ),
        8,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::C, LoadByteSource::A),
            LoadOperation::None,
        ),
        0,
    ),
    // 5X
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::D, LoadByteSource::B),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::D, LoadByteSource::C),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::D, LoadByteSource::D),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::D, LoadByteSource::E),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::D, LoadByteSource::H),
            LoadOperation::None,
        ),
        4,
    ),
    (Instruction::UNDEFINED, 0),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::D, LoadByteSource::MHL),
            LoadOperation::None,
        ),
        8,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::D, LoadByteSource::A),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::E, LoadByteSource::B),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::E, LoadByteSource::C),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::E, LoadByteSource::D),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::E, LoadByteSource::E),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::E, LoadByteSource::H),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::E, LoadByteSource::L),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::E, LoadByteSource::MHL),
            LoadOperation::None,
        ),
        8,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::E, LoadByteSource::A),
            LoadOperation::None,
        ),
        4,
    ),
    // 6X
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::H, LoadByteSource::B),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::H, LoadByteSource::C),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::H, LoadByteSource::D),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::H, LoadByteSource::E),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::H, LoadByteSource::H),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::H, LoadByteSource::L),
            LoadOperation::None,
        ),
        4,
    ),
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
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::L, LoadByteSource::C),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::L, LoadByteSource::D),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::L, LoadByteSource::E),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::L, LoadByteSource::H),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::L, LoadByteSource::L),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::L, LoadByteSource::MHL),
            LoadOperation::None,
        ),
        8,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::L, LoadByteSource::A),
            LoadOperation::None,
        ),
        0,
    ),
    // 7X
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MHL, LoadByteSource::B),
            LoadOperation::None,
        ),
        8,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MHL, LoadByteSource::C),
            LoadOperation::None,
        ),
        8,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MHL, LoadByteSource::D),
            LoadOperation::None,
        ),
        8,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MHL, LoadByteSource::E),
            LoadOperation::None,
        ),
        8,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MHL, LoadByteSource::H),
            LoadOperation::None,
        ),
        8,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MHL, LoadByteSource::L),
            LoadOperation::None,
        ),
        8,
    ),
    (Instruction::UNDEFINED, 4),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::MHL, LoadByteSource::A),
            LoadOperation::None,
        ),
        8,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::B),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::C),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::D),
            LoadOperation::None,
        ),
        4,
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
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::L),
            LoadOperation::None,
        ),
        4,
    ),
    (
        Instruction::LD(
            LoadType::Byte(LoadByteTarget::A, LoadByteSource::MHL),
            LoadOperation::None,
        ),
        8,
    ),
    (Instruction::UNDEFINED, 0),
    // 8X
    (
        Instruction::ADD(ArithmeticType::Byte(ArithmeticByteTarget::B)),
        4,
    ),
    (
        Instruction::ADD(ArithmeticType::Byte(ArithmeticByteTarget::C)),
        4,
    ),
    (
        Instruction::ADD(ArithmeticType::Byte(ArithmeticByteTarget::D)),
        4,
    ),
    (
        Instruction::ADD(ArithmeticType::Byte(ArithmeticByteTarget::E)),
        4,
    ),
    (
        Instruction::ADD(ArithmeticType::Byte(ArithmeticByteTarget::H)),
        4,
    ),
    (
        Instruction::ADD(ArithmeticType::Byte(ArithmeticByteTarget::L)),
        4,
    ),
    (
        Instruction::ADD(ArithmeticType::Byte(ArithmeticByteTarget::MHL)),
        8,
    ),
    (
        Instruction::ADD(ArithmeticType::Byte(ArithmeticByteTarget::A)),
        4,
    ),
    (Instruction::ADC(ArithmeticByteTarget::B), 4),
    (Instruction::ADC(ArithmeticByteTarget::C), 4),
    (Instruction::ADC(ArithmeticByteTarget::D), 4),
    (Instruction::ADC(ArithmeticByteTarget::E), 4),
    (Instruction::ADC(ArithmeticByteTarget::H), 4),
    (Instruction::ADC(ArithmeticByteTarget::L), 4),
    (Instruction::ADC(ArithmeticByteTarget::MHL), 8),
    (Instruction::ADC(ArithmeticByteTarget::A), 4),
    // 9X
    (Instruction::SUB(ArithmeticByteTarget::B), 4),
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
    (Instruction::AND(ArithmeticByteTarget::B), 4),
    (Instruction::AND(ArithmeticByteTarget::C), 4),
    (Instruction::AND(ArithmeticByteTarget::D), 4),
    (Instruction::AND(ArithmeticByteTarget::E), 4),
    (Instruction::AND(ArithmeticByteTarget::H), 4),
    (Instruction::AND(ArithmeticByteTarget::L), 4),
    (Instruction::AND(ArithmeticByteTarget::MHL), 8),
    (Instruction::AND(ArithmeticByteTarget::A), 4),
    (Instruction::XOR(ArithmeticByteTarget::B), 4),
    (Instruction::XOR(ArithmeticByteTarget::C), 4),
    (Instruction::XOR(ArithmeticByteTarget::D), 4),
    (Instruction::XOR(ArithmeticByteTarget::E), 4),
    (Instruction::XOR(ArithmeticByteTarget::H), 4),
    (Instruction::XOR(ArithmeticByteTarget::L), 4),
    (Instruction::XOR(ArithmeticByteTarget::MHL), 8),
    (Instruction::XOR(ArithmeticByteTarget::A), 4),
    // BX
    (Instruction::OR(ArithmeticByteTarget::B), 4),
    (Instruction::OR(ArithmeticByteTarget::C), 4),
    (Instruction::OR(ArithmeticByteTarget::D), 4),
    (Instruction::OR(ArithmeticByteTarget::E), 4),
    (Instruction::OR(ArithmeticByteTarget::H), 4),
    (Instruction::OR(ArithmeticByteTarget::L), 4),
    (Instruction::OR(ArithmeticByteTarget::MHL), 8),
    (Instruction::OR(ArithmeticByteTarget::A), 4),
    (Instruction::CP(ArithmeticByteTarget::B), 4),
    (Instruction::CP(ArithmeticByteTarget::C), 4),
    (Instruction::CP(ArithmeticByteTarget::D), 4),
    (Instruction::CP(ArithmeticByteTarget::E), 4),
    (Instruction::CP(ArithmeticByteTarget::H), 4),
    (Instruction::CP(ArithmeticByteTarget::L), 4),
    (Instruction::CP(ArithmeticByteTarget::MHL), 8),
    (Instruction::CP(ArithmeticByteTarget::A), 4),
    // CX
    (Instruction::RET(JumpCondition::NZ), 0),
    (Instruction::POP(ArithmeticWordTarget::BC), 0),
    (Instruction::JP(JumpCondition::NZ, JumpTarget::N16), 0),
    (Instruction::JP(JumpCondition::NONE, JumpTarget::N16), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::PUSH(ArithmeticWordTarget::BC), 0),
    (
        Instruction::ADD(ArithmeticType::Byte(ArithmeticByteTarget::N8)),
        8,
    ),
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
    (Instruction::RET(JumpCondition::NC), 0),
    (Instruction::POP(ArithmeticWordTarget::DE), 0),
    (Instruction::JP(JumpCondition::NC, JumpTarget::N16), 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::UNDEFINED, 0),
    (Instruction::PUSH(ArithmeticWordTarget::DE), 16),
    (Instruction::SUB(ArithmeticByteTarget::N8), 8),
    (Instruction::UNDEFINED, 0),
    (Instruction::RET(JumpCondition::C), 4),
    (Instruction::RETI, 16),
    (Instruction::JP(JumpCondition::C, JumpTarget::N16), 4),
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
    (Instruction::DI, 4),
    (Instruction::UNDEFINED, 0),
    (Instruction::PUSH(ArithmeticWordTarget::AF), 16),
    (Instruction::OR(ArithmeticByteTarget::N8), 8),
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
    (Instruction::RLC, 8),
    (Instruction::RRC, 8),
    (Instruction::RL, 8),
    (Instruction::RR, 8),
    (Instruction::SLA, 8),
    (Instruction::SRA, 8),
    (Instruction::SWAP, 8),
    (Instruction::SRL, 8),
    (Instruction::BIT(0), 8),
    (Instruction::BIT(1), 8),
    (Instruction::BIT(2), 8),
    (Instruction::BIT(3), 8),
    (Instruction::BIT(4), 8),
    (Instruction::BIT(5), 8),
    (Instruction::BIT(6), 8),
    (Instruction::BIT(7), 8),
    (Instruction::RES(0), 8),
    (Instruction::RES(1), 8),
    (Instruction::RES(2), 8),
    (Instruction::RES(3), 8),
    (Instruction::RES(4), 8),
    (Instruction::RES(5), 8),
    (Instruction::RES(6), 8),
    (Instruction::RES(7), 8),
    (Instruction::SET(0), 8),
    (Instruction::SET(1), 8),
    (Instruction::SET(2), 8),
    (Instruction::SET(3), 8),
    (Instruction::SET(4), 8),
    (Instruction::SET(5), 8),
    (Instruction::SET(6), 8),
    (Instruction::SET(7), 8),
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

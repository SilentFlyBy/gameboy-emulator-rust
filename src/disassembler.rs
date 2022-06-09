use std::collections::{hash_map::Entry, HashMap};

use crate::{
    bus::{Bus, FetchWrite},
    cpu::{
        helper::parse_prefix_instruction,
        instructions::{
            ArithmeticByteTarget, ArithmeticType, ArithmeticWordTarget, Instruction, JumpCondition,
            JumpTarget, LoadByteSource, LoadByteTarget, LoadOperation, LoadType, LoadWordSource,
            LoadWordTarget,
        },
    },
};

pub struct Disassembler {
    disassembly: HashMap<u16, String>,
}

impl Disassembler {
    pub fn new() -> Self {
        Disassembler {
            disassembly: HashMap::new(),
        }
    }

    pub fn disassemble(&mut self, bus: &mut Bus, instruction: Instruction, program_counter: u16) {
        let disassembly = match instruction {
            Instruction::PREFIX => {
                let prefix_code = bus.fetch8(program_counter + 1).unwrap();
                let ((prefix_instruction, cycles), target) = parse_prefix_instruction(prefix_code);

                match self.disassembly.entry(program_counter) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => v.insert(disassemble_prefix_instruction(
                        prefix_instruction,
                        target,
                        bus,
                        program_counter,
                    )),
                }
            }
            _ => match self.disassembly.entry(program_counter) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => {
                    v.insert(disassemble_instruction(instruction, bus, program_counter))
                }
            },
        };

        println!("{:#06X}: {}", program_counter, disassembly);
    }
}

pub fn disassemble_instruction(i: Instruction, bus: &mut Bus, pc: u16) -> String {
    return match i {
        Instruction::NOP => String::from("NOP"),
        Instruction::EI => String::from("EI"),
        Instruction::DI => String::from("DI"),
        Instruction::RETI => String::from("RETI"),
        Instruction::RLA => String::from("RLA"),
        Instruction::PUSH(target) => {
            let target_string = get_arithmetic_word_target_string(target);

            format!("PUSH {}", target_string)
        }
        Instruction::POP(target) => {
            let target_string = get_arithmetic_word_target_string(target);

            format!("POP {}", target_string)
        }
        Instruction::CPL => String::from("CPL"),
        Instruction::RST(address) => format!("RST {:#X}", address),
        Instruction::ADD(arithmetic_type) => {
            let target_string = match arithmetic_type {
                ArithmeticType::Byte(target) => get_arithmetic_byte_target_string(target, bus, pc),
                ArithmeticType::Word(target) => {
                    String::from(get_arithmetic_word_target_string(target))
                }
            };

            format!("ADD {}", target_string)
        }
        Instruction::SUB(target) => {
            let target_string = get_arithmetic_byte_target_string(target, bus, pc);

            format!("SUB {}", target_string)
        }
        Instruction::CP(target) => {
            let target_string = get_arithmetic_byte_target_string(target, bus, pc);

            format!("CP {}", target_string)
        }
        Instruction::INC(arithmetic_type) => {
            let target_string = match arithmetic_type {
                ArithmeticType::Byte(target) => get_arithmetic_byte_target_string(target, bus, pc),
                ArithmeticType::Word(target) => {
                    String::from(get_arithmetic_word_target_string(target))
                }
            };

            format!("INC {}", target_string)
        }
        Instruction::DEC(arithmetic_type) => {
            let target_string = match arithmetic_type {
                ArithmeticType::Byte(target) => get_arithmetic_byte_target_string(target, bus, pc),
                ArithmeticType::Word(target) => {
                    String::from(get_arithmetic_word_target_string(target))
                }
            };

            format!("DEC {}", target_string)
        }
        Instruction::AND(target) => {
            let target_string = get_arithmetic_byte_target_string(target, bus, pc);

            format!("AND {}", target_string)
        }
        Instruction::OR(target) => {
            let target_string = get_arithmetic_byte_target_string(target, bus, pc);

            format!("OR {}", target_string)
        }
        Instruction::XOR(target) => {
            let target_string = get_arithmetic_byte_target_string(target, bus, pc);

            format!("XOR {}", target_string)
        }
        Instruction::JP(condition, target) => {
            let target_string = match target {
                JumpTarget::HL => String::from("HL"),
                JumpTarget::N16 => {
                    let value = bus.fetch16(pc + 1).unwrap();

                    format!("{:#X}", value)
                }
            };

            let condition_string = get_jump_condition_string(condition);

            format!("JP {}{}", condition_string, target_string)
        }
        Instruction::JR(condition) => {
            let condition_string = get_jump_condition_string(condition);
            let value = bus.fetch8(pc).unwrap();

            format!("JR {}{:#X}", condition_string, value)
        }
        Instruction::CALL(condition) => {
            let condition_string = get_jump_condition_string(condition);
            let value = bus.fetch16(pc + 1).unwrap();

            format!("CALL {}{:#X}", condition_string, value)
        }
        Instruction::RET(condition) => {
            let condition_string = get_jump_condition_string(condition);

            format!("RET {}", condition_string)
        }
        Instruction::LD(LoadType::Byte(target, source), operation) => {
            let source_string = match source {
                LoadByteSource::A => String::from("A"),
                LoadByteSource::B => String::from("B"),
                LoadByteSource::C => String::from("C"),
                LoadByteSource::D => String::from("D"),
                LoadByteSource::E => String::from("E"),
                LoadByteSource::H => String::from("H"),
                LoadByteSource::L => String::from("L"),
                LoadByteSource::MDE => String::from("[DE]"),
                LoadByteSource::MHL => match operation {
                    LoadOperation::HLI => String::from("[HL+]"),
                    LoadOperation::HLD => String::from("[HL-]"),
                    LoadOperation::None => String::from("[HL]"),
                },
                LoadByteSource::N8 => {
                    let value = bus.fetch8(pc).unwrap();

                    format!("{:#X}", value)
                }
                LoadByteSource::DN8 => {
                    let value = bus.fetch8(pc).unwrap();

                    format!("[#FF00 + {:#X}]", value)
                }
                LoadByteSource::DC => String::from("[#FF00 + C]"),
                LoadByteSource::MN16 => {
                    let value = bus.fetch16(pc + 1).unwrap();

                    format!("[{:#X}]", value)
                }
            };

            let target_string = match target {
                LoadByteTarget::A => String::from("A"),
                LoadByteTarget::B => String::from("B"),
                LoadByteTarget::C => String::from("C"),
                LoadByteTarget::D => String::from("D"),
                LoadByteTarget::E => String::from("E"),
                LoadByteTarget::H => String::from("H"),
                LoadByteTarget::L => String::from("L"),
                LoadByteTarget::MBC => String::from("[BC]"),
                LoadByteTarget::MDE => String::from("[DE]"),
                LoadByteTarget::MHL => match operation {
                    LoadOperation::HLI => String::from("[HL+]"),
                    LoadOperation::HLD => String::from("[HL-]"),
                    LoadOperation::None => String::from("[HL]"),
                },
                LoadByteTarget::MN16 => {
                    let value = bus.fetch16(pc + 1).unwrap();

                    format!("[{:#X}]", value)
                }
                LoadByteTarget::DN8 => {
                    let value = bus.fetch8(pc).unwrap();

                    format!("[#FF00 + {:#X}]", value)
                }
                LoadByteTarget::DC => String::from("[#FF00 + C]"),
            };

            format!("LD {}, {}", target_string, source_string)
        }
        Instruction::LD(LoadType::Word(target, source), operation) => {
            let source_string = match source {
                LoadWordSource::N16 => {
                    let value = bus.fetch16(pc + 1).unwrap();

                    format!("{:#X}", value)
                }
            };

            let target_string = match target {
                LoadWordTarget::HL => match operation {
                    LoadOperation::HLI => "[HL+]",
                    LoadOperation::HLD => "[HL-]",
                    LoadOperation::None => "[HL]",
                },
                LoadWordTarget::SP => "SP",
                LoadWordTarget::BC => "BC",
                LoadWordTarget::DE => "DE",
            };

            format!("LD {}, {}", target_string, source_string)
        }
        _ => String::from(""),
    };
}

pub fn disassemble_prefix_instruction(
    instruction: Instruction,
    target: ArithmeticByteTarget,
    bus: &mut Bus,
    pc: u16,
) -> String {
    return match instruction {
        Instruction::SWAP => {
            let target_string = get_arithmetic_byte_target_string(target, bus, pc);

            format!("SWAP {}", target_string)
        }
        Instruction::SET(n) => {
            let target_string = get_arithmetic_byte_target_string(target, bus, pc);

            format!("SET {}, {}", n, target_string)
        }
        Instruction::RL => {
            let target_string = get_arithmetic_byte_target_string(target, bus, pc);

            format!("RL {}", target_string)
        }
        Instruction::RES(n) => {
            let target_string = get_arithmetic_byte_target_string(target, bus, pc);

            format!("RES {}, {}", n, target_string)
        }
        Instruction::BIT(n) => {
            let target_string = get_arithmetic_byte_target_string(target, bus, pc);

            format!("BIT {}, {}", n, target_string)
        }
        _ => String::from(""),
    };
}

fn get_arithmetic_byte_target_string(
    target: ArithmeticByteTarget,
    bus: &mut Bus,
    pc: u16,
) -> String {
    match target {
        ArithmeticByteTarget::A => String::from("A"),
        ArithmeticByteTarget::B => String::from("B"),
        ArithmeticByteTarget::C => String::from("C"),
        ArithmeticByteTarget::D => String::from("D"),
        ArithmeticByteTarget::E => String::from("E"),
        ArithmeticByteTarget::H => String::from("H"),
        ArithmeticByteTarget::L => String::from("L"),
        ArithmeticByteTarget::MHL => String::from("[HL]"),
        ArithmeticByteTarget::N8 => {
            let value = bus.fetch8(pc).unwrap();

            format!("{:#X}", value)
        }
    }
}

fn get_arithmetic_word_target_string(target: ArithmeticWordTarget) -> &'static str {
    match target {
        ArithmeticWordTarget::BC => "BC",
        ArithmeticWordTarget::HL => "HL",
        ArithmeticWordTarget::AF => "AF",
        ArithmeticWordTarget::DE => "DE",
    }
}

fn get_jump_condition_string(condition: JumpCondition) -> &'static str {
    match condition {
        JumpCondition::Z => "Z, ",
        JumpCondition::NZ => "NZ, ",
        JumpCondition::C => "C, ",
        JumpCondition::NC => "NC, ",
        JumpCondition::NONE => "",
    }
}

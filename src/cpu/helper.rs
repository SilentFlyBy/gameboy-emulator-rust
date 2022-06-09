use super::instructions::{ArithmeticByteTarget, Instruction, PREFIX_CODES, PREFIX_TARGETS};

pub fn parse_prefix_instruction(opcode: u8) -> ((Instruction, u8), ArithmeticByteTarget) {
    // first 3 bytes identify the target
    let target_bits = opcode & 0x7;

    // last 5 bytes identify the instruction
    let instruction_bits = opcode >> 3;

    let (instruction, cycles) = PREFIX_CODES[instruction_bits as usize];
    let target = PREFIX_TARGETS[target_bits as usize];

    ((instruction, cycles), target)
}

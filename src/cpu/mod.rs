use std::{
    collections::{hash_map::Entry, HashMap},
    io::{self, ErrorKind},
};

use crate::{
    bus::{Bus, FetchWrite},
    cpu::instructions::OPCODES,
    disassembler::Disassembler,
};

use self::{
    helper::parse_prefix_instruction,
    instructions::{
        ArithmeticByteTarget, ArithmeticType, ArithmeticWordTarget, Instruction, JumpCondition,
        JumpTarget, LoadByteSource, LoadByteTarget, LoadOperation, LoadType, LoadWordSource,
        LoadWordTarget, PREFIX_CODES, PREFIX_TARGETS,
    },
};

pub mod helper;
pub mod instructions;

pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    program_counter: u16,
    stack_pointer: u16,
    disassembler: Disassembler,
}

const ZERO_FLAG_MASK: u8 = 1 << 7;
const SUBSTRACTION_FLAG_MASK: u8 = 1 << 6;
const HALFCARRY_FLAG_MASK: u8 = 1 << 5;
const CARRY_FLAG_MASK: u8 = 1 << 4;

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0x1,
            b: 0x0,
            c: 0x13,
            d: 0x0,
            e: 0xD8,
            f: 0,
            h: 0x1,
            l: 0x4D,
            program_counter: 0x100,
            stack_pointer: 0xFFFE,
            disassembler: Disassembler::new(),
        }
    }
    pub fn next(&mut self, bus: &mut Bus) -> std::io::Result<u8> {
        if bus.interrupts.interrupt_pending() {
            self.handle_interrupt(bus);
        }
        let opcode = self.next_byte(bus)?;
        let (instruction, cycles) = self.decode_instruction(bus, opcode)?;

        match self.run_instruction(bus, instruction) {
            Err(_) => {
                self.unwind_stack(bus);
                panic!("Unsupported opcode {:#X}", opcode)
            }
            Ok(()) => {}
        };
        bus.next(cycles);

        Ok(cycles)
    }

    fn decode_instruction(
        &mut self,
        bus: &mut Bus,
        opcode: u8,
    ) -> std::io::Result<(Instruction, u8)> {
        let (instruction, cycles) = OPCODES[opcode as usize];

        /*self.disassembler
        .disassemble(bus, instruction, self.program_counter - 1);*/

        Ok((instruction, cycles))
    }

    fn run_instruction(&mut self, bus: &mut Bus, instruction: Instruction) -> io::Result<()> {
        match instruction {
            Instruction::NOP => {}
            Instruction::LD(load_type, load_operation) => self.ld(bus, load_type, load_operation),
            Instruction::JP(condition, target) => self.jp(bus, condition, target),
            Instruction::JR(condition) => self.jr(bus, condition),
            Instruction::CALL(condition) => self.call(bus, condition),
            Instruction::RET(condition) => self.ret(bus, condition),
            Instruction::RST(address) => self.rst(bus, address),
            Instruction::PUSH(target) => self.push(bus, target),
            Instruction::POP(target) => self.pop(bus, target),
            Instruction::OR(target) => self.or(bus, target),
            Instruction::XOR(target) => self.xor(bus, target),
            Instruction::AND(target) => self.and(bus, target),
            Instruction::INC(arithmetic_type) => self.inc(bus, arithmetic_type),
            Instruction::DEC(arithmetic_type) => self.dec(bus, arithmetic_type),
            Instruction::CP(target) => self.cp(bus, target),
            Instruction::ADD(arithmetic_type) => self.add(bus, arithmetic_type),
            Instruction::ADC(target) => self.adc(bus, target),
            Instruction::SUB(target) => self.sub(bus, target),
            Instruction::RLA => self.rla(),
            Instruction::RLCA => self.rlca(bus),
            Instruction::CCF => self.ccf(),
            Instruction::CPL => self.cpl(bus),
            Instruction::DAA => self.daa(bus),
            Instruction::EI => self.ei(bus),
            Instruction::DI => self.di(bus),
            Instruction::RETI => self.reti(bus),
            Instruction::PREFIX => self.prefix(bus),
            _ => return Err(io::Error::new(ErrorKind::Unsupported, "error")),
        }
        Ok(())
    }

    fn handle_interrupt(&mut self, bus: &mut Bus) {
        bus.interrupts.disable_master();
        let handler_address = bus.interrupts.ack_and_get_pending_address();
        if handler_address.is_none() {
            return;
        }

        let address = handler_address.unwrap();
        self.push16(bus, self.program_counter);
        self.set_program_counter(address);
    }

    fn next_byte(&mut self, bus: &mut Bus) -> std::io::Result<u8> {
        let val = bus.fetch8(self.program_counter);
        self.program_counter += 0x1;

        val
    }

    fn next_word(&mut self, bus: &mut Bus) -> std::io::Result<u16> {
        let val = bus.fetch16(self.program_counter);
        self.program_counter += 0x2;

        val
    }

    fn af(&self) -> u16 {
        return ((self.a as u16) << 8) | (self.f as u16);
    }

    fn hl(&self) -> u16 {
        return ((self.h as u16) << 8) | (self.l as u16);
    }

    fn bc(&self) -> u16 {
        return ((self.b as u16) << 8) | (self.c as u16);
    }

    fn de(&self) -> u16 {
        return ((self.d as u16) << 8) | (self.e as u16);
    }

    fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    fn set_l(&mut self, value: u8) {
        self.l = value;
    }

    fn set_bc(&mut self, value: u16) {
        let b = ((value & 0xFF00) >> 8) as u8;
        let c = (value & 0x00FF) as u8;

        self.set_b(b);
        self.set_c(c);
    }

    fn set_de(&mut self, value: u16) {
        let d = ((value & 0xFF00) >> 8) as u8;
        let e = (value & 0x00FF) as u8;

        self.set_d(d);
        self.set_e(e);
    }

    fn set_hl(&mut self, value: u16) {
        let h = ((value & 0xFF00) >> 8) as u8;
        let l = (value & 0x00FF) as u8;

        self.set_h(h);
        self.set_l(l);
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00F0) as u8;
    }

    fn set_stack_pointer(&mut self, value: u16) {
        self.stack_pointer = value;
    }

    fn set_program_counter(&mut self, value: u16) {
        self.program_counter = value;
    }

    fn carry(&self) -> bool {
        (self.f & CARRY_FLAG_MASK) > 0
    }

    fn set_carry(&mut self, value: bool) {
        if value {
            self.f |= CARRY_FLAG_MASK
        } else {
            self.f &= !CARRY_FLAG_MASK;
        }
    }

    fn halfcarry(&self) -> bool {
        (self.f & HALFCARRY_FLAG_MASK) > 0
    }

    fn set_halfcarry(&mut self, value: bool) {
        if value {
            self.f |= HALFCARRY_FLAG_MASK
        } else {
            self.f &= !HALFCARRY_FLAG_MASK;
        }
    }

    fn subtract(&self) -> bool {
        (self.f & SUBSTRACTION_FLAG_MASK) > 0
    }

    fn set_subtract(&mut self, value: bool) {
        if value {
            self.f |= SUBSTRACTION_FLAG_MASK
        } else {
            self.f &= !SUBSTRACTION_FLAG_MASK;
        }
    }

    fn zero(&self) -> bool {
        (self.f & ZERO_FLAG_MASK) > 0
    }

    fn set_zero(&mut self, value: bool) {
        if value {
            self.f |= ZERO_FLAG_MASK
        } else {
            self.f &= !ZERO_FLAG_MASK;
        }
    }

    fn push8(&mut self, bus: &mut Bus, value: u8) {
        self.set_stack_pointer(self.stack_pointer - 1);
        bus.write8(self.stack_pointer, value).unwrap();
    }

    fn push16(&mut self, bus: &mut Bus, value: u16) {
        self.push8(bus, (value >> 8) as u8);
        self.push8(bus, value as u8);
    }

    fn pop8(&mut self, bus: &mut Bus) -> u8 {
        let value = bus.fetch8(self.stack_pointer).unwrap();
        self.set_stack_pointer(self.stack_pointer + 1);

        value
    }

    fn pop16(&mut self, bus: &mut Bus) -> u16 {
        let lo = self.pop8(bus) as u16;
        let hi = self.pop8(bus) as u16;

        (hi << 8) | lo
    }

    fn push(&mut self, bus: &mut Bus, target: ArithmeticWordTarget) {
        let value = self.read_arithmetic_word_target(bus, target);
        self.push16(bus, value);
    }

    fn pop(&mut self, bus: &mut Bus, target: ArithmeticWordTarget) {
        let value = self.pop16(bus);
        self.write_arithmetic_word_target(bus, target, value);
    }

    fn call(&mut self, bus: &mut Bus, condition: JumpCondition) {
        let address = self.next_word(bus).unwrap();

        if !self.jump_condition_met(condition) {
            return;
        }

        self.push16(bus, self.program_counter);
        self.set_program_counter(address);
    }

    fn daa(&mut self, bus: &mut Bus) {
        let mut adjust = 0;

        if self.halfcarry() || self.carry() {
            adjust |= 0x06;
        }

        let result = if self.subtract() {
            self.a.wrapping_sub(adjust)
        } else {
            if self.a & 0x0F > 0x09 {
                adjust |= 0x06;
            }

            if self.a > 0x99 {
                adjust |= 0x60;
            }

            self.a.wrapping_add(adjust)
        };

        self.set_a(result);

        self.set_zero(result == 0);
        self.set_carry(adjust & 0x60 != 0);
        self.set_halfcarry(false);
    }

    fn ret(&mut self, bus: &mut Bus, condition: JumpCondition) {
        if !self.jump_condition_met(condition) {
            return;
        }

        let address = self.pop16(bus);
        self.set_program_counter(address);
    }

    fn rlca(&mut self, bus: &mut Bus) {
        let c = self.a >> 7;
        self.set_a((self.a << 1) | c);

        self.set_carry(c != 0);
        self.set_zero(false);
        self.set_halfcarry(false);
        self.set_subtract(false);
    }

    fn rst(&mut self, bus: &mut Bus, address: u16) {
        self.push16(bus, self.program_counter);
        self.set_program_counter(address)
    }

    fn rla(&mut self) {
        self.set_carry(self.a & 0x80 == 0x80);
        let r = (self.a << 1) | (if self.carry() { 1 } else { 0 });
        self.set_a(r);

        self.set_zero(false);
        self.set_subtract(false);
        self.set_halfcarry(false);
    }

    fn rl(&mut self, bus: &mut Bus, target: ArithmeticByteTarget) {
        let value = self.read_arithmetic_byte_target(bus, target);

        let c = value & 0x80 == 0x80;
        let r = (value << 1) | (if self.carry() { 1 } else { 0 });

        self.set_halfcarry(false);
        self.set_subtract(false);
        self.set_zero(r == 0);
        self.set_carry(c);
    }

    fn bit(&mut self, bus: &mut Bus, target: ArithmeticByteTarget, n: u8) {
        let value = self.read_arithmetic_byte_target(bus, target);

        let r = value & (1 << (n as u32)) == 0;
        self.set_subtract(false);
        self.set_halfcarry(false);
        self.set_zero(r);
    }

    fn prefix(&mut self, bus: &mut Bus) {
        let opcode = self.next_byte(bus).unwrap();

        let ((instruction, cycles), target) = parse_prefix_instruction(opcode);

        match instruction {
            Instruction::SWAP => self.swap(bus, target),
            Instruction::SET(n) => self.set(bus, target, n),
            Instruction::RES(n) => self.res(bus, target, n),
            Instruction::BIT(n) => self.bit(bus, target, n),
            Instruction::RL => self.rl(bus, target),
            Instruction::SLA => self.sla(bus, target),
            Instruction::SRL => self.srl(bus, target),
            _ => panic!("Prefix Instruction unsupported: {:#X}", opcode),
        }
    }

    fn ccf(&mut self) {
        self.set_carry(!self.carry());
        self.set_subtract(false);
        self.set_halfcarry(false);
    }

    fn srl(&mut self, bus: &mut Bus, target: ArithmeticByteTarget) {
        let value = self.read_arithmetic_byte_target(bus, target);
        self.set_carry(value & 1 != 0);

        let result = value >> 1;
        self.set_zero(result == 0);
        self.set_subtract(false);
        self.set_halfcarry(false);

        self.write_arithmetic_byte_target(bus, target, result);
    }

    fn sla(&mut self, bus: &mut Bus, target: ArithmeticByteTarget) {
        let value = self.read_arithmetic_byte_target(bus, target);
        self.set_carry(value & 0x80 != 0);

        let result = value << 1;
        self.set_zero(result == 0);
        self.set_subtract(false);
        self.set_halfcarry(false);

        self.write_arithmetic_byte_target(bus, target, result);
    }

    fn set(&mut self, bus: &mut Bus, target: ArithmeticByteTarget, n: u8) {
        let value = self.read_arithmetic_byte_target(bus, target);
        let result = value | (1 << n);
        self.write_arithmetic_byte_target(bus, target, result);
    }

    fn res(&mut self, bus: &mut Bus, target: ArithmeticByteTarget, n: u8) {
        let value = self.read_arithmetic_byte_target(bus, target);
        let result = value & !(1 << n);
        self.write_arithmetic_byte_target(bus, target, result);
    }

    fn swap(&mut self, bus: &mut Bus, target: ArithmeticByteTarget) {
        let value = self.read_arithmetic_byte_target(bus, target);

        let result = (value >> 4) | (value << 4);
        self.write_arithmetic_byte_target(bus, target, result);

        self.set_zero(result == 0);
        self.set_subtract(false);
        self.set_halfcarry(false);
        self.set_carry(false);
    }

    fn cpl(&mut self, bus: &mut Bus) {
        self.set_a(!self.a);

        self.set_subtract(true);
        self.set_halfcarry(true);
    }

    fn adc(&mut self, bus: &mut Bus, target: ArithmeticByteTarget) {
        let value = self.read_arithmetic_byte_target(bus, target);

        let x = self.a as u32;
        let y = value as u32;
        let carry = self.carry() as u32;

        let result = x.wrapping_add(y).wrapping_add(carry);
        let rb = result as u8;

        self.set_zero(rb == 0);
        self.set_halfcarry((x ^ y ^ result) & 0x10 != 0);
        self.set_carry(result & 0x100 != 0);
        self.set_subtract(false);

        self.set_a(rb)
    }

    fn add(&mut self, bus: &mut Bus, arithmetic_type: ArithmeticType) {
        match arithmetic_type {
            ArithmeticType::Byte(target) => {
                let a = self.a;
                let value = self.read_arithmetic_byte_target(bus, target);

                let (new_value, did_overflow) = a.overflowing_add(value);
                self.set_a(new_value);

                self.set_zero(new_value == 0);
                self.set_subtract(false);
                self.set_carry(did_overflow);
                // Half Carry is set if adding the lower nibbles of the value and register A
                // together result in a value bigger than 0xF. If the result is larger than 0xF
                // than the addition caused a carry from the lower nibble to the upper nibble.
                self.set_halfcarry((a & 0xF) + (value & 0xF) > 0xF);
            }
            ArithmeticType::Word(target) => {
                let hl = self.hl();
                let value = self.read_arithmetic_word_target(bus, target);

                let (new_value, did_overflow) = self.hl().overflowing_add(value);
                self.set_hl(new_value);

                self.set_zero(false);
                self.set_subtract(false);
                self.set_carry(did_overflow);
                self.set_halfcarry((hl & 0x07FF) + (value & 0x07FF) > 0x07FF)
            }
        }
    }

    fn sub(&mut self, bus: &mut Bus, target: ArithmeticByteTarget) {
        let value = self.read_arithmetic_byte_target(bus, target);

        let (new_value, did_overflow) = self.a.overflowing_sub(value);
        self.set_a(new_value);

        self.set_zero(new_value == 0);
        self.set_subtract(true);
        self.set_carry(did_overflow);
        self.set_halfcarry((self.a & 0xF) < (value & 0xF));
    }

    fn cp(&mut self, bus: &mut Bus, target: ArithmeticByteTarget) {
        let a = self.a;
        self.sub(bus, target);
        self.set_a(a);
    }

    fn inc(&mut self, bus: &mut Bus, arithmetic_type: ArithmeticType) {
        match arithmetic_type {
            ArithmeticType::Byte(target) => {
                let value = self.read_arithmetic_byte_target(bus, target);

                self.set_halfcarry((value & 0x0F) + 1 > 0x0F);

                let new_value = value.wrapping_add(1);
                self.write_arithmetic_byte_target(bus, target, new_value);

                self.set_zero(new_value == 0);
                self.set_subtract(false);
            }
            ArithmeticType::Word(target) => {
                let value = self.read_arithmetic_word_target(bus, target);
                self.write_arithmetic_word_target(bus, target, value.wrapping_add(1));
            }
        }
    }

    fn dec(&mut self, bus: &mut Bus, arithmetic_type: ArithmeticType) {
        match arithmetic_type {
            ArithmeticType::Byte(target) => {
                let value = self.read_arithmetic_byte_target(bus, target);

                self.set_halfcarry(value & 0xf == 0);

                let new_value = value.wrapping_sub(1);
                self.write_arithmetic_byte_target(bus, target, new_value);

                self.set_zero(new_value == 0);
                self.set_subtract(true);
            }
            ArithmeticType::Word(target) => {
                let value = self.read_arithmetic_word_target(bus, target);
                self.write_arithmetic_word_target(bus, target, value.wrapping_sub(1));
            }
        }
    }

    fn or(&mut self, bus: &mut Bus, target: ArithmeticByteTarget) {
        let value = self.read_arithmetic_byte_target(bus, target);

        let result = self.a | value;
        self.set_a(result);

        self.set_zero(result == 0);
        self.set_subtract(false);
        self.set_carry(false);
        self.set_halfcarry(false);
    }

    fn xor(&mut self, bus: &mut Bus, target: ArithmeticByteTarget) {
        let value = self.read_arithmetic_byte_target(bus, target);

        let result = self.a ^ value;
        self.set_a(result);
        self.set_zero(result == 0);
        self.set_carry(false);
        self.set_halfcarry(false);
        self.set_subtract(false);
    }

    fn and(&mut self, bus: &mut Bus, target: ArithmeticByteTarget) {
        let value = self.read_arithmetic_byte_target(bus, target);
        let new_value = self.a & value;
        self.set_a(new_value);

        self.set_zero(new_value == 0);
        self.set_subtract(false);
        self.set_carry(false);
        self.set_halfcarry(true);
    }

    fn ld(&mut self, bus: &mut Bus, load_type: LoadType, load_operation: LoadOperation) {
        match load_type {
            LoadType::Byte(target, source) => {
                let source_value = match source {
                    LoadByteSource::A => self.a,
                    LoadByteSource::B => self.b,
                    LoadByteSource::C => self.c,
                    LoadByteSource::D => self.d,
                    LoadByteSource::E => self.e,
                    LoadByteSource::H => self.h,
                    LoadByteSource::L => self.l,
                    LoadByteSource::MHL => bus.fetch8(self.hl()).unwrap(),
                    LoadByteSource::MBC => bus.fetch8(self.bc()).unwrap(),
                    LoadByteSource::MDE => bus.fetch8(self.de()).unwrap(),
                    LoadByteSource::N8 => self.next_byte(bus).unwrap(),
                    LoadByteSource::DN8 => {
                        let mut address = self.next_byte(bus).unwrap() as u16;
                        address |= 0xFF00;
                        bus.fetch8(address).unwrap()
                    }
                    LoadByteSource::DC => {
                        let mut address = self.c as u16;
                        address |= 0xFF00;
                        bus.fetch8(address).unwrap()
                    }
                    LoadByteSource::MN16 => {
                        let address = self.next_word(bus).unwrap();
                        bus.fetch8(address).unwrap()
                    }
                };

                match target {
                    LoadByteTarget::A => self.set_a(source_value),
                    LoadByteTarget::B => self.set_b(source_value),
                    LoadByteTarget::C => self.set_c(source_value),
                    LoadByteTarget::D => self.set_d(source_value),
                    LoadByteTarget::E => self.set_e(source_value),
                    LoadByteTarget::H => self.set_h(source_value),
                    LoadByteTarget::L => self.set_l(source_value),
                    LoadByteTarget::MHL => bus.write8(self.hl(), source_value).unwrap(),
                    LoadByteTarget::MBC => bus.write8(self.bc(), source_value).unwrap(),
                    LoadByteTarget::MDE => bus.write8(self.de(), source_value).unwrap(),
                    LoadByteTarget::MN16 => {
                        let address = self.next_word(bus).unwrap();
                        bus.write8(address, source_value).unwrap();
                    }
                    LoadByteTarget::DN8 => {
                        let n = self.next_byte(bus).unwrap();
                        let address = n as u16 | 0xFF00;
                        bus.write8(address, source_value).unwrap();
                    }
                    LoadByteTarget::DC => {
                        let address = self.c as u16 | 0xFF00;
                        bus.write8(address, source_value).unwrap();
                    }
                };
            }
            LoadType::Word(target, source) => {
                let source_value = match source {
                    LoadWordSource::N16 => self.next_word(bus).unwrap(),
                    LoadWordSource::SP => self.stack_pointer,
                };
                match target {
                    LoadWordTarget::HL => self.set_hl(source_value),
                    LoadWordTarget::SP => self.set_stack_pointer(source_value),
                    LoadWordTarget::BC => self.set_bc(source_value),
                    LoadWordTarget::DE => self.set_de(source_value),
                    LoadWordTarget::MN16 => {
                        let address = self.next_word(bus).unwrap();
                        bus.write16(address, source_value).unwrap();
                    }
                }
            }
        }

        match load_operation {
            LoadOperation::HLI => self.set_hl(self.hl().wrapping_add(1)),
            LoadOperation::HLD => self.set_hl(self.hl().wrapping_sub(1)),
            LoadOperation::None => {}
        }
    }

    fn jp(&mut self, bus: &mut Bus, condition: JumpCondition, target: JumpTarget) {
        let address = match target {
            JumpTarget::N16 => self.next_word(bus).unwrap(),
            JumpTarget::HL => self.hl(),
        };

        if !self.jump_condition_met(condition) {
            return;
        }

        self.program_counter = address;
    }

    fn jr(&mut self, bus: &mut Bus, condition: JumpCondition) {
        let offset = self.next_byte(bus).unwrap() as i8;

        if !self.jump_condition_met(condition) {
            return;
        }

        let mut pc = self.program_counter as i16;
        pc = pc.wrapping_add(offset as i16);
        self.program_counter = pc as u16;
    }

    fn ei(&mut self, bus: &mut Bus) {
        bus.interrupts.enable_master();
    }

    fn di(&mut self, bus: &mut Bus) {
        bus.interrupts.disable_master();
    }

    fn reti(&mut self, bus: &mut Bus) {
        self.ei(bus);
        self.ret(bus, JumpCondition::NONE);
    }

    fn read_arithmetic_byte_target(&mut self, bus: &mut Bus, target: ArithmeticByteTarget) -> u8 {
        return match target {
            ArithmeticByteTarget::A => self.a,
            ArithmeticByteTarget::B => self.b,
            ArithmeticByteTarget::C => self.c,
            ArithmeticByteTarget::D => self.d,
            ArithmeticByteTarget::E => self.e,
            ArithmeticByteTarget::H => self.h,
            ArithmeticByteTarget::L => self.l,
            ArithmeticByteTarget::MHL => bus.fetch8(self.hl()).unwrap(),
            ArithmeticByteTarget::N8 => self.next_byte(bus).unwrap(),
        };
    }

    fn write_arithmetic_byte_target(
        &mut self,
        bus: &mut Bus,
        target: ArithmeticByteTarget,
        value: u8,
    ) {
        match target {
            ArithmeticByteTarget::A => self.set_a(value),
            ArithmeticByteTarget::B => self.set_b(value),
            ArithmeticByteTarget::C => self.set_c(value),
            ArithmeticByteTarget::D => self.set_d(value),
            ArithmeticByteTarget::E => self.set_e(value),
            ArithmeticByteTarget::H => self.set_h(value),
            ArithmeticByteTarget::L => self.set_l(value),
            ArithmeticByteTarget::MHL => bus.write8(self.hl(), value).unwrap(),
            ArithmeticByteTarget::N8 => {}
        }
    }

    fn read_arithmetic_word_target(&mut self, bus: &mut Bus, target: ArithmeticWordTarget) -> u16 {
        return match target {
            ArithmeticWordTarget::BC => self.bc(),
            ArithmeticWordTarget::HL => self.hl(),
            ArithmeticWordTarget::AF => self.af(),
            ArithmeticWordTarget::DE => self.de(),
        };
    }

    fn write_arithmetic_word_target(
        &mut self,
        bus: &mut Bus,
        target: ArithmeticWordTarget,
        value: u16,
    ) {
        match target {
            ArithmeticWordTarget::AF => self.set_af(value),
            ArithmeticWordTarget::BC => self.set_bc(value),
            ArithmeticWordTarget::HL => self.set_hl(value),
            ArithmeticWordTarget::DE => self.set_de(value),
        }
    }

    fn jump_condition_met(&self, condition: JumpCondition) -> bool {
        match condition {
            JumpCondition::Z => self.zero(),
            JumpCondition::NZ => !self.zero(),
            JumpCondition::C => self.carry(),
            JumpCondition::NC => !self.carry(),
            JumpCondition::NONE => true,
        }
    }

    fn unwind_stack(&self, bus: &mut Bus) {
        println!("Program counter: {:#X}", self.program_counter - 1);
        println!("Stack trace:");
        for address in (0xFFFE..self.stack_pointer).step_by(2) {
            println!("{:#X}", bus.fetch16(address).unwrap());
        }
    }

    fn print_memory(&self, bus: &mut Bus, start_addr: u16, end_addr: u16) {
        for address in start_addr..=end_addr {
            let value = bus.fetch8(address).unwrap();
            println!("{:#X}: {:#X}", address, value);
        }
    }
}

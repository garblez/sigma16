use std::convert::From;

use crate::util;

// For now, this will focus on the Core ISA (Instruction Set Architecture)
// I'm only one person and this is the first time I've done it so give me a break. ;)
type R = u16;  // Index of register (0-15)
type C = u16; // Constant value

#[derive(Debug)]
pub enum Instruction {
    Add(R, R, R),
    Sub(R, R, R),
    Mul(R, R, R),
    Div(R, R, R),
    Cmp(R, R),
    Trap(R, R, R),
    Lea(R, C),
    Load(R, C),
    Store(R, C),
    Jump(C),
    Jumpc0(C),
    Jumpc1(C),
    Jal(C),
}

fn get_opcode(instr: Instruction) -> u16 {
    use Instruction::*;

    match instr {
        Add(_, _, _) => 0x0,
        Sub(_, _, _) => 0x1,
        Mul(_, _, _) => 0x2,
        Div(_, _, _) => 0x3,
        Cmp(_, _) => 0x4,
        Trap(_, _, _) => 0x5,
        Lea(_, _) => 0x6,
        Load(_, _) => 0x7,
        _ => 0xf // Don't have an implementation for these other instructions yet
    }
}

enum InstructionType {
    RRR, EXP, RX
}

fn get_type(word: u16) -> InstructionType {
    use InstructionType::*;
    match (word & 0xf000) >> 12 {
        0xe => EXP,
        0xf => RX,
        _   => RRR,
    }
}

impl From<u16> for Instruction {
    fn from(word: u16) -> Self {
        let (opcode, dest, a, b) = util::read(word);
        let addr = 0x0000;
        use Instruction::*;

        match opcode {
            0x0 => Add(dest, a, b),
            0x1 => Sub(dest, a, b),
            0x2 => Mul(dest, a, b),
            0x3 => Div(dest, a, b),
            0x4 => Cmp(a, b),
            0x5 => Trap(dest, a, b),
            0x6 => Lea(dest, a),
            0x7 => Load(dest, addr),
            _ => Trap(0, 0, 0)
        }
    }
}

use std::convert::From;

use std::fs::File;
use std::io;
use std::io::prelude::*;

// For now, this will focus on the Core ISA (Instruction Set Architecture)
// I'm only one person and this is the first time I've done it so give me a break. ;)
type R = u16;  // Index of register (0-15)
type C = u16; // Constant value

#[derive(Debug)]
enum Instruction {
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

fn read(word: u16) -> (u16, u16, u16, u16) {
    ((word & 0xf000) >> 12, (word & 0x0f00) >> 8, (word & 0x00f0) >> 4, word & 0x000f)
}

impl From<u16> for Instruction {
    fn from(word: u16) -> Self {
        let (opcode, dest, a, b) = read(word);
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


#[derive(Debug)]
enum State {
    Initialising,
    Ready,
    Executing,
    Halt,
}

enum ConditionCode {
    IntegerGreaterThan,
    NaturalGreaterThan,
    Equals,
    NaturalLessThan,
    IntegerLessThan,
    IntegerOverflow,
    NaturalOverflow,
    Carry,
    StackOverflow,
    StackUnderflow,
    LogicFunctionResult,
}

impl From<ConditionCode> for u8 {
    fn from(condition: ConditionCode) -> Self {
        use ConditionCode::*;
        match condition {
            IntegerGreaterThan => 0,
            NaturalGreaterThan => 1,
            Equals => 2,
            NaturalLessThan => 3,
            IntegerLessThan => 4,
            IntegerOverflow => 5,
            NaturalOverflow => 6,
            Carry => 7,
            StackOverflow => 8,
            StackUnderflow => 9,
            LogicFunctionResult => 10,
        }
    }
}


#[derive(Debug)]
struct Processor {
    register: [u16; 16],
    // Instruction control registers
    pc: u16,    // Program counter
    ir: u16,    // Instruction register
    adr: u16,   // Address register
    dat: u16,   // Data register
    // Interrupt control registers
    state: State,
    memory: [u16; 65537],
    mask: u16,
    req: u16,
    rstat: u16,
    rpc: u16,
    vect: u16,
    sys: bool,
    ie: bool,
}


impl Processor {
    fn new() -> Self {
        Processor {
            register: [0; 16],
            pc: 0,
            ir: 0,
            adr: 0,
            dat: 0,
            state: State::Initialising,
            memory: [0; 65537],
            mask: 0,
            req: 0,
            rstat: 0,
            rpc: 0,
            vect: 0,
            sys: false,
            ie: false,
        }
    }

    fn load_program(&mut self, filepath: &str) -> io::Result<()> {
        let mut f = File::open(filepath)?;
        let mut buffer = Vec::<u8>::new();

        f.read_to_end(&mut buffer)?;

        if buffer.len() / 2 > 65537 {
            return Ok(()) // not strictly `Ok` but better we just invalidate memory by leaving it empty so the processor can halt on the next loop
        }

        for (index, pair) in buffer.chunks_exact(2).enumerate() {
            self.memory[index] = (pair[0] as u16) << 8 | pair[1] as u16;
        };

        Ok(())
    }

    fn load_bytes(&mut self, bytes: Vec<u8>) -> Result<(), &'static str> {

        if bytes.len() / 2 > 65537 {
             Err("The number of bytes you supplied would exceed the available memory on a Sigma16 system")
        } else {

            for (index, pair) in bytes.chunks_exact(2).enumerate() {
                self.memory[index] = (pair[0] as u16) << 8 | pair[1] as u16;
            };
            Ok(())
        }
    }

    fn set_register_bit(&mut self, register: usize, bit: u8, value: bool) {
        let flag = if value {
            1u16
        } else {
            0u16
        };

        self.register[register] |= flag << bit;
    }

    fn set_condition_code(&mut self, condition_code: ConditionCode, flag: bool) {
        let bit_index: u8 = condition_code.into();

        self.set_register_bit(15, bit_index, flag);
    }

}

fn main() {
    let cpu = Processor::new();
    println!("{:?}", cpu.state);
}


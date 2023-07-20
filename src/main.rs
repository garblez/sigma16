use ux::{u4, u12};

// For now, this will focus on the Core ISA (Instruction Set Architecture)
// I'm only one person and this is the first time I've done it so give me a break. ;)
type R = u4;  // Index of register (0-15)
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
    Jump(R, C),
    Jumpc0(R, C),
    Jumpc1(R, C),
    Jal(R, C),
}


#[derive(Debug)]
enum State {
    Initialising,
    Ready,
    Executing(Instruction),
    Halt,
}


#[derive(Debug)]
struct Processor {
    register_file: [u16; 16],
    // Instruction control registers
    pc: u16,    // Program counter
    ir: u16,    // Instruction register
    adr: u16,   // Address register
    dat: u16,   // Data register
    // Interrupt control registers
    state: State
}


impl Processor {
    fn new() -> Self {
        Processor {
            register_file: [0; 16],
            pc: 0,
            ir: 0,
            adr: 0,
            dat: 0,
            state: State::Initialising 
        }
    }

    fn issue(&mut self, word: u16) -> Instruction {
        // This is one simple step if the instruction is only the length of a word,
        // otherwise, we need to increment the pc and read the second half of the 
        // instruction before execution.
        interpret(word) // Perhaps we interpret the binary in this and get an instruction outwith the processor ?
    }
}

fn main() {
    let cpu = Processor::new();
    let x: u4 = u4::new(15);
    println!("{:?}", cpu.state);
    println!("{:1x}", x);
}


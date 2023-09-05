use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::state::State;
use crate::condition_code::ConditionCode;

#[derive(Debug)]
pub struct Processor {
    register: [u16; 16],
    // Instruction control registers
    pc: u16,    // Program counter
    ir: u16,    // Instruction register
    adr: u16,   // Address register
    dat: u16,   // Data register
    // Interrupt control registers
    pub state: State,
    pub memory: [u16; 65537],
    mask: u16,
    req: u16,
    rstat: u16,
    rpc: u16,
    vect: u16,
    sys: bool,
    ie: bool,
}


impl Processor {
    pub fn new() -> Self {
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

    pub fn load_program(&mut self, filepath: &str) -> io::Result<()> {
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

    pub fn load_bytes(&mut self, bytes: Vec<u8>) -> Result<(), &'static str> {

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

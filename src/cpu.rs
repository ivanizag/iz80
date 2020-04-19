use super::decoder::*;
use super::environment::*;
use super::machine::*;
use super::state::*;

pub struct Cpu {
    decoder: Decoder,
    trace: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            decoder: Decoder::new(),
            trace: false
        }
    }

    pub fn execute_instruction(&mut self, state: &mut State, sys: &mut dyn Machine) {
        if self.trace {
            let pc = state.reg.get_pc();
            let opcode_index = sys.peek(pc);
            //print!("==== {:04x}: {:02x} ", pc, opcode_index);
            print!("==== {:04x}: {:02x} {:02x} {:02x} ", pc, opcode_index,
                sys.peek(pc+1), sys.peek(pc+2));
        }
        let mut env = Environment::new(state, sys);
        let opcode = self.decoder.decode(&mut env);
        if self.trace {
            println!("{}", opcode.disasm(&mut env));
        }
        opcode.execute(&mut env);
        env.step();
    }

    pub fn set_trace(&mut self, trace: bool) {
        self.trace = trace;
    }
}



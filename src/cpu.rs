use super::decoder::*;
use super::environment::*;
use super::machine::*;
use super::registers::*;
use super::state::*;

/// The Z80 cpu emulator.
/// 
/// Executes Z80 instructions changing the cpu State and Machine
pub struct Cpu {
    decoder: Decoder,
    trace: bool,
}

impl Cpu {
    /// Returns a Cpu instance
    pub fn new() -> Cpu {
        Cpu {
            decoder: Decoder::new(),
            trace: false
        }
    }

    /// Executes a single Z80 instruction
    /// 
    /// # Arguments
    /// 
    /// * `state` - The internal state of the Z80
    /// * `sys` - A representation of the emulated machine that has the Machine trait
    ///  
    pub fn execute_instruction(&mut self, state: &mut State, sys: &mut dyn Machine) {
        if self.trace {
            let pc = state.reg.pc();
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

        if self.trace {
            // CPU registers
            println!("PC({:04x}) AF({:04x}) BC({:04x}) DE({:04x}) HL({:04x}) SP({:04x}) IX({:04x}) IY({:04x}) Flags({:08b})",
                state.reg.pc(),
                state.reg.get16(Reg16::AF),
                state.reg.get16(Reg16::BC),
                state.reg.get16(Reg16::DE),
                state.reg.get16(Reg16::HL),
                state.reg.get16(Reg16::SP),
                state.reg.get16(Reg16::IX),
                state.reg.get16(Reg16::IY),
                state.reg.get8(Reg8::F)
            );
        }
    }

    /// Activates or deactivates traces of the instruction executed and
    /// the state of the registers.
    /// 
    /// # Arguments
    /// 
    /// * `trace` - A bool defining the trace state to set
    pub fn set_trace(&mut self, trace: bool) {
        self.trace = trace;
    }
}



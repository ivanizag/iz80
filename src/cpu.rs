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
    state: State,
    trace: bool,
}

impl Cpu {
    /// Returns a Cpu instance
    pub fn new() -> Cpu {
        Cpu {
            decoder: Decoder::new(),
            state: State::new(),
            trace: false
        }
    }

    /// Executes a single Z80 instruction
    /// 
    /// # Arguments
    /// 
    /// * `sys` - A representation of the emulated machine that has the Machine trait
    ///  
    pub fn execute_instruction(&mut self, sys: &mut dyn Machine) {
        if self.trace {
            let pc = self.state.reg.pc();
            let opcode_index = sys.peek(pc);
            //print!("==== {:04x}: {:02x} ", pc, opcode_index);
            print!("==== {:04x}: {:02x} {:02x} {:02x} ", pc, opcode_index,
                sys.peek(pc+1), sys.peek(pc+2));
        }

        let mut env = Environment::new(&mut self.state, sys);
        let opcode = self.decoder.decode(&mut env);
        if self.trace {
            println!("{}", opcode.disasm(&mut env));
        }
        opcode.execute(&mut env);
        env.step();

        if self.trace {
            // CPU registers
            println!("PC({:04x}) AF({:04x}) BC({:04x}) DE({:04x}) HL({:04x}) SP({:04x}) IX({:04x}) IY({:04x}) Flags({:08b})",
                self.state.reg.pc(),
                self.state.reg.get16(Reg16::AF),
                self.state.reg.get16(Reg16::BC),
                self.state.reg.get16(Reg16::DE),
                self.state.reg.get16(Reg16::HL),
                self.state.reg.get16(Reg16::SP),
                self.state.reg.get16(Reg16::IX),
                self.state.reg.get16(Reg16::IY),
                self.state.reg.get8(Reg8::F)
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

    /// Returns a Registers struct to read and write on the Z80 registers
    pub fn registers(&mut self) -> &mut Registers {
        &mut self.state.reg
    }
}



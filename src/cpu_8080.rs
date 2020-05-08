use super::decoder_8080::*;
use super::environment::*;
use super::machine::*;
use super::registers::*;
use super::state::*;

/// The 8080 cpu emulator.
/// 
/// Executes Intel 8080 instructions changing the cpu State and Machine
pub struct Cpu8080 {
    decoder: Decoder8080,
    state: State,
    trace: bool,
}

impl Cpu8080 {
    /// Returns a Cpu instance
    pub fn new() -> Cpu8080 {
        let mut cpu = Cpu8080 {
            decoder: Decoder8080::new(),
            state: State::new(),
            trace: false
        };

        cpu.state.reg.set_8080();
        cpu
    }

    /// Executes a single Z80 instruction
    /// 
    /// # Arguments
    /// 
    /// * `sys` - A representation of the emulated machine that has the Machine trait
    ///  
    pub fn execute_instruction(&mut self, sys: &mut dyn Machine) {
        let pc = self.state.reg.pc();
        let mut env = Environment::new(&mut self.state, sys);
        let opcode = self.decoder.decode(&mut env);
        if self.trace {
            println!("==> {:04x}: {:20} ", pc, opcode.disasm(&mut env));
        }
        opcode.execute(&mut env);
        env.step();

        if self.trace {
            println!("PC:{:04x} AF:{:04x} BC:{:04x} DE:{:04x} HL:{:04x} SP:{:04x}",  // Flags:{:08b}",
                self.state.reg.pc(),
                self.state.reg.get16(Reg16::AF),
                self.state.reg.get16(Reg16::BC),
                self.state.reg.get16(Reg16::DE),
                self.state.reg.get16(Reg16::HL),
                self.state.reg.get16(Reg16::SP),
                // self.state.reg.get8(Reg8::F)
            );
            //println!(" [{:02x} {:02x} {:02x}]", sys.peek(pc+1),
            //    sys.peek(pc+1), sys.peek(pc+2));
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

    /// Returns if the Cpu has executed a HALT
    pub fn is_halted(&self) -> bool {
        self.state.halted
    }
}



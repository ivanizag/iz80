use super::decoder_z80::*;
use super::decoder_8080::*;
use super::environment::*;
use super::machine::*;
use super::opcode::*;
use super::registers::*;
use super::state::*;

const NMI_ADDRESS: u16 = 0x0066;

/// The Z80 cpu emulator.
/// 
/// Executes Z80 instructions changing the cpu State and Machine
pub struct Cpu {
    state: State,
    trace: bool,
    decoder: Box<dyn Decoder>,
}

pub(crate) trait Decoder {
    fn decode(&self, env: &mut Environment) -> &Opcode;
}

impl Cpu {

    /// Returns a Z80 Cpu instance. Alias of new_z80()
    pub fn new() -> Cpu {
        Cpu {
            state: State::new(),
            trace: false,
            decoder: Box::new(DecoderZ80::new())
        }
    }

    /// Returns a Z80 Cpu instance
    pub fn new_z80() -> Cpu {
        Cpu {
            state: State::new(),
            trace: false,
            decoder: Box::new(DecoderZ80::new())
        }
    }

    /// Returns an Intel 8080 Cpu instance
    pub fn new_8080() -> Cpu {
        let mut cpu = Cpu {
            state: State::new(),
            trace: false,
            decoder: Box::new(Decoder8080::new())
        };

        cpu.state.reg.set_8080();
        cpu
    }


    /// Executes a single instruction
    ///
    /// # Arguments
    ///
    /// * `sys` - A representation of the emulated machine that has the Machine trait
    ///
    pub fn execute_instruction(&mut self, sys: &mut dyn Machine) {
        if self.is_halted() {
            // The CPU is in HALT state. Only interrupts can execute.
            return
        }

        let mut env = Environment::new(&mut self.state, sys);
        if env.state.reset_pending {
            env.state.reset_pending = false;
            env.state.nmi_pending = false;
            env.state.halted = false;
            env.state.reg.set_pc(0x0000);
            env.state.reg.set8(Reg8::I, 0x00);
            env.state.reg.set8(Reg8::R, 0x00);
            env.state.reg.set_interrupts(false);
            env.state.reg.set_interrupt_mode(0);
        }
        else if env.state.nmi_pending {
            env.state.nmi_pending = false;
            env.state.halted = false;
            env.state.reg.start_nmi();
            env.subroutine_call(NMI_ADDRESS);
        }

        let pc = env.state.reg.pc();
        let opcode = self.decoder.decode(&mut env);
        if self.trace {
            print!("==> {:04x}: {:20}", pc, opcode.disasm(&mut env));
        }
        opcode.execute(&mut env);
        env.clear_index();

        if self.trace {
            print!(" PC:{:04x} AF:{:04x} BC:{:04x} DE:{:04x} HL:{:04x} SP:{:04x} IX:{:04x} IY:{:04x} Flags:{:08b}",
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
            println!(" [{:02x} {:02x} {:02x}]", sys.peek(pc),
                sys.peek(pc.wrapping_add(1)), sys.peek(pc.wrapping_add(2)));
        }
    }

    /// Returns the instrction in PC disassembled. PC is advanced.
    /// 
    /// # Arguments
    /// 
    /// * `sys` - A representation of the emulated machine that has the Machine trait
    ///  
    pub fn disasm_instruction(&mut self, sys: &mut dyn Machine) -> String {
        let mut env = Environment::new(&mut self.state, sys);
        let opcode = self.decoder.decode(&mut env);
        opcode.disasm(&mut env)
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
        self.state.halted && !self.state.nmi_pending && !self.state.reset_pending
    }

    /// Non maskable interrupt request
    pub fn signal_nmi(&mut self) {
        self.state.nmi_pending = true
    }

    /// Signal reset
    pub fn signal_reset(&mut self) {
        self.state.reset_pending = true
    }
}



#![warn(missing_docs)]
//#![warn(missing_doc_code_examples)]

//! Z80 Emulator library that passess all ZEXALL tests
//! 
//! See cpuville.rs or the iz-cpm project for complete use examples.
//! 
//!# Example
//! ```
//! // Prepare host
//! let mut machine = PlainMachine::new();
//! let mut state = State::new();
//! let mut cpu = Cpu::new();
//! 
//!  // Load program
//! let code = include_bytes!("xxx.bin");
//! for i in 0..code.len() {
//!     machine.poke(0x0000 + i as u16, code[i]);
//! }
//! 
//! // Run the emulation
//! state.reg.set_pc(0x0000);
//! loop {
//!     cpu.execute_instruction(&mut state, &mut machine);
//! }
//! ```


mod cpu;
mod machine;
mod registers;
mod state;


mod decoder;
mod environment;
mod opcode;
mod opcode_alu;
mod opcode_arith;
mod opcode_bits;
mod opcode_io;
mod opcode_jumps;
mod opcode_ld;
mod operators;

pub use cpu::Cpu;
pub use machine::Machine;
pub use machine::PlainMachine;
pub use registers::*;
pub use state::State;

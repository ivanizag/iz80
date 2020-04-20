#![warn(missing_docs)]
//! Z80 Emulator library that passess all ZEXALL tests
//! 
//! See cpuville.rs or the [iz-cpm](https://github.com/ivanizag/iz-cpm)
//! project for usage examples.
//! 
//!# Example
//! To run ths example, execute: `cargo run --bin simplest`
//! 
//! ```
//!use iz80::*;
//!
//!fn main() {
//!    // Prepare the device
//!    let mut machine = PlainMachine::new();
//!    let mut cpu = Cpu::new();
//!    cpu.set_trace(true);
//!
//!    // Load program inline or from a file with:
//!    //      let code = include_bytes!("XXXX.rom");
//!    let code = [0x3c, 0xc3, 0x00, 0x00]; // INC A, JP $0000
//!    let size = code.len();
//!    for i in 0..size {
//!        machine.poke(0x0000 + i as u16, code[i]);
//!    }
//!
//!    // Run emulation
//!    cpu.registers().set_pc(0x0000);
//!    loop {
//!        cpu.execute_instruction(&mut machine);
//!
//!        // Examine Machine state to update the hosting device as needed.
//!        if cpu.registers().a() == 0x10 {
//!            // Let's stop
//!            break;
//!        }
//!    }
//!}
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
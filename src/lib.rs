#![warn(missing_docs)]
//! Z80 Emulator library that passes all ZEXALL tests
//! 
//! See bin/cpuville.rs or the [iz-cpm](https://github.com/ivanizag/iz-cpm)
//! project for usage examples.
//! 
//!# Example
//! To run this example, execute: `cargo run --bin simplest`
//! 
//! ```
//!use iz80::*;
//!
//!fn main() {
//!    // Prepare the device
//!    let mut machine = PlainMachine::new();
//!    let mut cpu = Cpu::new(); // Or Cpu8080::new()
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
//!        // Examine machine state to update the hosting device as needed.
//!        if cpu.registers().a() == 0x10 {
//!            // Let's stop
//!            break;
//!        }
//!    }
//!}
//! ```


mod cpu;
mod cpu_8080;
mod machine;
mod registers;
mod state;


mod decoder_z80;
mod decoder_8080;
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
pub use cpu_8080::Cpu8080;
pub use machine::Machine;
pub use machine::PlainMachine;
pub use registers::*;
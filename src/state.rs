use std::io;

use super::registers::{Reg16, Registers};

/// Internal state of the CPU
/// 
/// Stores the state of the registers and additional hidden execution
/// state of the CPU.
pub struct State {
    /// Values of the Z80 registers
    pub reg: Registers,
    /// Cycle counter
    pub cycle: u64,
    pub branch_taken: bool,
    /// Halt state of the CPU
    pub halted: bool,
    /// Maskable interrupt signaled
    pub int_signaled: bool,
    /// Non maskable interrupt signaled
    pub nmi_pending: bool,
    /// Reset signaled
    pub reset_pending: bool,
    /// Interrupts just enabled
    pub int_just_enabled: bool,
    // Alternate index management
    pub index: Reg16, // Using HL, IX or IY
    pub displacement: i8, // Used for (IX+d) and (iY+d)
}

impl State {
    /// Returns the initial state of a Z80 on power up
    pub fn new() -> State {
        State {
            reg: Registers::new(),
            cycle: 0,
            branch_taken: false,
            halted: false,
            int_signaled: false,
            nmi_pending: false,
            reset_pending: false,
            int_just_enabled: false,
            index: Reg16::HL,
            displacement: 0,
        }
    }

    pub const SERIALIZE_SIZE: usize = Registers::SERIALIZE_SIZE + 8 + 8;

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(State::SERIALIZE_SIZE);
        data.extend_from_slice(&self.reg.serialize());
        data.extend_from_slice(&self.cycle.to_le_bytes());
        data.push(self.branch_taken as u8);
        data.push(self.halted as u8);
        data.push(self.int_signaled as u8);
        data.push(self.nmi_pending as u8);
        data.push(self.reset_pending as u8);
        data.push(self.int_just_enabled as u8);
       match self.index {
            Reg16::IX => data.push(1),
            Reg16::IY => data.push(2),
            _ => data.push(0),
        }
        data.push(self.displacement as u8);
        data
    }

    pub fn deserialize(&mut self, data: &[u8]) -> io::Result<()> {
        if data.len() < State::SERIALIZE_SIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Data too short"));
        }
        let err = self.reg.deserialize(&data[0..]);
        err?;

        let i = Registers::SERIALIZE_SIZE;
        self.cycle = u64::from_le_bytes(data[i..i+8].try_into().unwrap());
        self.branch_taken = data[i+8] != 0;
        self.halted = data[i+9] != 0;
        self.int_signaled = data[i+10] != 0;
        self.nmi_pending = data[i+11] != 0;
        self.reset_pending = data[i+12] != 0;
        self.int_just_enabled = data[i+13] != 0;
        match data[i+14] {
            1 => self.index = Reg16::IX,
            2 => self.index = Reg16::IY,
            _ => self.index = Reg16::HL,
        }
        self.displacement = data[i+15] as i8;
        Ok(())
    }
}

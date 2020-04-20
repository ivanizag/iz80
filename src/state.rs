use super::registers::*;

/// Internal state of the CPU
/// 
/// Stores the state of the registers and additional hidden execution
/// state of the CPU.
pub struct State {
    /// Values of the Z80 registers
    pub reg: Registers,
    /// Halt state of the CPU
    pub halted: bool,
    // Alternate index management
    pub index: Reg16, // Using HL, IX or IY
    pub displacement: i8, // Used for (IX+d) and (iY+d)
    pub displacement_loaded: bool, // TODO: remove
    pub index_changed: bool, // Use the index change for the next opcode, reset afterwards
}

impl State {
    /// Returns the initial state of a Z80 on power up
    pub fn new() -> State {
        State {
            reg: Registers::new(),
            halted: false,
            index: Reg16::HL,
            displacement: 0,
            displacement_loaded: false,
            index_changed: false
        }
    }
}

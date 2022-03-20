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
    /// Non maskable interrupt signaled
    pub nmi_pending: bool,
    // Alternate index management
    pub index: Reg16, // Using HL, IX or IY
    pub displacement: i8, // Used for (IX+d) and (iY+d)
    pub displacement_loaded: bool, // TODO: remove
}

impl State {
    /// Returns the initial state of a Z80 on power up
    pub fn new() -> State {
        State {
            reg: Registers::new(),
            halted: false,
            nmi_pending: false,
            index: Reg16::HL,
            displacement: 0,
            displacement_loaded: false,
        }
    }
}

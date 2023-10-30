use super::registers::*;

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
    pub int_pending: bool,
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
            int_pending: false,
            nmi_pending: false,
            reset_pending: false,
            int_just_enabled: false,
            index: Reg16::HL,
            displacement: 0,
        }
    }
}

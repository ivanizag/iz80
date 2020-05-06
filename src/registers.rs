use std::fmt;

/// 8 bit registers
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Reg8 {
    /// 8 bit register A
    A = 0,
    /// 8 bit register F, can be accessed vif the flags methods
    F = 1, // Flags
    /// 8 bit register B
    B = 2,
    /// 8 bit register C
    C = 3,
    /// 8 bit register D
    D = 4,
    /// 8 bit register E
    E = 5,
    /// 8 bit register H, high byte of HL
    H = 6,
    /// 8 bit register L, low byte of HL
    L = 7,
    /// 8 bit register I
    I = 8,
    /// 8 bit register R
    R = 9,
    /// High byte of IX
    IXH = 10,
    /// Low byte of IX
    IXL = 11,
    /// High byte of IY
    IYH = 12,
    /// Low byte of IY
    IYL = 13,
    /// High byte of SP
    SPH = 14,
    /// Low byte of SP
    SPL = 15,
    /// Pseudo register, has to be replaced by (HL) 
     _HL = 16 // Invalid
}
const REG_COUNT8: usize = 16;


/// 16 bit registers, composed from 8 bit registers
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Reg16 {
    /// 16 but register AF 
    AF = Reg8::A as isize,
    /// 16 but register BC 
    BC = Reg8::B as isize,
    /// 16 but register DE 
    DE = Reg8::D as isize,
    /// 16 but register HL 
    HL = Reg8::H as isize,
    /// 16 but register IX 
    IX = Reg8::IXH as isize,
    /// 16 but register IY 
    IY = Reg8::IYH as isize,
    /// 16 but register SP
    SP = Reg8::SPH as isize
}

/// Z80 flags
#[derive(Copy, Clone, Debug)]
pub enum Flag {
    /// Carry flag
    C  = 1,
    /// Negative flag
    N  = 2,
    /// Parity or overflow flag
    P  = 4, // P/V
    /// Undocumented third flag
    _3 = 8,
    /// Half carry flag
    H  = 16,
    /// Undocumented fifth flag
    _5 = 32,
    /// Zero flag
    Z  = 64,
    /// Sign flag
    S  = 128
}

impl fmt::Display for Reg8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Reg8::_HL => write!(f, "(HL)"),
            _ => write!(f, "{:?}", *self)
        }
    }
}

/// Z80 internal register values
#[derive(Debug)]
pub struct Registers {
    data: [u8; REG_COUNT8],
    shadow: [u8; REG_COUNT8],
    pc: u16,
    iff1: bool,
    iff2: bool,
    im: u8
}

impl Registers {
    pub(crate) fn new() -> Registers {
        //Init z80 registers (TUZD-2.4)
        let mut reg = Registers {
            data: [0; REG_COUNT8],
            shadow: [0; REG_COUNT8],
            pc: 0,
            iff1: false,
            iff2: false,
            im: 0
        };

        reg.set16(Reg16::AF, 0xffff);
        reg.set16(Reg16::SP, 0xffff);

        reg
    }

    /// Returns the value of the A register
    #[inline]
    pub fn a(&self) -> u8 {
        self.data[Reg8::A as usize]
    }

    /// Sets the A register
    #[inline]
    pub fn set_a(&mut self, value: u8) {
        self.data[Reg8::A as usize] = value;
    }

    /// Returns the value of an 8 bit register
    #[inline]
    pub fn get8(&self, reg: Reg8) -> u8 {
        if reg == Reg8::_HL {
            panic!("Can't use the pseudo register (HL)");
        }
        self.data[reg as usize]
    }

    /// Sets the value of an 8 bit register
    #[inline]
    pub fn set8(&mut self, reg: Reg8, value: u8) {
        if reg == Reg8::_HL {
            panic!("Can't use the pseudo register (HL)");
        }
        self.data[reg as usize] = value;
    }

    pub(crate) fn inc_dec8(&mut self, reg: Reg8, inc: bool) -> u8 {
        let mut v = self.get8(reg);
        if inc {
            v = v.wrapping_add(1);
        } else {
            v = v.wrapping_sub(1);
        }
        self.set8(reg, v);
        v
    }

    /// Returns the value of a 16 bit register
    #[inline]
    pub fn get16(&self, rr: Reg16) -> u16 {
        self.data[rr as usize +1] as u16
        + ((self.data[rr as usize] as u16) << 8)
    }

    /// Sets the value of a 16 bit register. Changes the
    /// value of the two underlying 8 bit registers.
    #[inline]
    pub fn set16(&mut self, rr: Reg16, value: u16) {
        self.data[rr as usize +1] = value as u8;
        self.data[rr as usize] = (value >> 8) as u8;
    }

    pub(crate) fn inc_dec16(&mut self, rr: Reg16, inc: bool) -> u16 {
        let mut v = self.get16(rr);
        if inc {
            v = v.wrapping_add(1);
        } else {
            v = v.wrapping_sub(1);
        }
        self.set16(rr, v);
        v
    }

    pub(crate) fn swap(&mut self, rr: Reg16) {
        let ih = rr as usize;
        let temp = self.data[ih];
        self.data[ih] = self.shadow[ih];
        self.shadow[ih] = temp;

        let il = rr as usize + 1;
        let temp = self.data[il];
        self.data[il] = self.shadow[il];
        self.shadow[il] = temp;
    }

    /// Returns the value of a flag
    pub fn get_flag(&self, flag: Flag) -> bool {
        self.get8(Reg8::F) & flag as u8 != 0
    }

    /// Sets a flag. Sets the value to true
    pub fn set_flag(&mut self, flag: Flag) {
        self.data[Reg8::F as usize] |= flag as u8;
    }

    /// Clears a flag. Sets the value to false
    pub fn clear_flag(&mut self, flag: Flag) {
        self.data[Reg8::F as usize] &= !(flag as u8);
    }

    /// Sets the value of a flag
    pub fn put_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }



    // Only for add18
    pub(crate) fn update_ch_flags(&mut self, xored: u16) {
        let carry_bit = (xored >> 8 & 1) != 0;
        self.put_flag(Flag::C, carry_bit);

        let half_bit  = (xored >> 4 & 1) != 0;
        self.put_flag(Flag::H, half_bit);
    }

    fn update_p_flag(&mut self, reference: u8) {
        let bits = reference.count_ones();
        self.put_flag(Flag::P, bits % 2 == 0);
    }

    pub(crate) fn update_undocumented_flags(&mut self, reference: u8) {
        let f: &mut u8 = &mut self.data[Reg8::F as usize];

        // Bits 5, and 3 are copied
        const MASK_53: u8 = Flag::_5 as u8 + Flag::_3 as u8;
        *f = (*f & !MASK_53) + (reference & MASK_53);
    }

    fn update_sz53_flags(&mut self, reference: u8) {
        self.update_undocumented_flags(reference);

        let f: &mut u8 = &mut self.data[Reg8::F as usize];
        // Zero
        if reference == 0 {
            *f |= Flag::Z as u8
        } else {
            *f &= !(Flag::Z as u8)
        }

        // Sign is copied
        const MASK_S: u8 = Flag::S as u8;
        *f = (*f & !MASK_S) + (reference & MASK_S);
    }

// sz5h3[pv]nc

    //sz5h3vnc
    pub(crate) fn update_arithmetic_flags(&mut self, reference: u8, xored: u16, neg: bool) {
        // TUZD-8.6
        let carry_bit = (xored >> 8 & 1) != 0;
        self.put_flag(Flag::C, carry_bit);

        self.update_inc_dec_flags(reference, xored, neg);
    }

    // sz5h3vn
    pub(crate) fn update_inc_dec_flags(&mut self, reference: u8, xored: u16, neg: bool) {
        self.update_sz53_flags(reference);

        let half_bit  = (xored >> 4 & 1) != 0;
        self.put_flag(Flag::H, half_bit);

        let carry_bit = (xored >> 8 & 1) != 0;
        let top_xor   = (xored >> 7 & 1) != 0;
        self.put_flag(Flag::P, carry_bit != top_xor); // As overflow flag

        self.put_flag(Flag::N, neg);
    }

    pub(crate) fn update_logic_flags(&mut self, reference: u8, is_and: bool) {
        self.update_sz53_flags(reference);

        self.update_p_flag(reference);
        self.clear_flag(Flag::C);
        self.clear_flag(Flag::N);
        self.put_flag(Flag::H, is_and);
    }

    pub(crate) fn update_block_flags(&mut self, reference: u8, k: u16, counter: u8) {
        // TUZD-4.3
        self.update_sz53_flags(counter);

        self.put_flag(Flag::H, k>255);
        self.update_p_flag(k as u8 & 7 ^ counter);
        self.put_flag(Flag::N, reference >> 7 == 1);
        self.put_flag(Flag::C, k>255);
    }

    pub(crate) fn update_bits_in_flags(&mut self, reference: u8) {
        self.update_sz53_flags(reference);
        self.update_p_flag(reference);
        self.clear_flag(Flag::H);
        self.clear_flag(Flag::N);
    }

    pub(crate) fn update_daa_flags(&mut self, reference: u8, hf: bool, cf:bool) {
        self.update_sz53_flags(reference);
        self.update_p_flag(reference);
        self.put_flag(Flag::H, hf);
        self.put_flag(Flag::C, cf);

    }

    /// Returns the program counter
    #[inline]
    pub fn pc(&self) -> u16 {
        self.pc
    }

    /// Changes the program counter
    #[inline]
    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub(crate) fn set_interrupts(&mut self, v: bool) {
        self.iff2 = v;
    }

    pub(crate) fn set_interrupt_mode(&mut self, im: u8) {
        self.im = im;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_8bit_register() {
        let mut r = Registers::new();
        const V:u8 = 23;

        r.set8(Reg8::A, V);
        assert_eq!(V, r.get8(Reg8::A));
    }

    #[test]
    fn set_get_16bit_register() {
        let mut r = Registers::new();

        r.set16(Reg16::BC, 0x34de);
        assert_eq!(0x34de, r.get16(Reg16::BC));
        assert_eq!(0x34, r.get8(Reg8::B));
        assert_eq!(0xde, r.get8(Reg8::C));
    }

    #[test]
    fn set_get_flag() {
        let mut r = Registers::new();
 
        r.set_flag(Flag::P);
        assert_eq!(true, r.get_flag(Flag::P));
        r.clear_flag(Flag::P);
        assert_eq!(false, r.get_flag(Flag::P));
        r.put_flag(Flag::P, true);
        assert_eq!(true, r.get_flag(Flag::P));
        r.put_flag(Flag::P, false);
        assert_eq!(false, r.get_flag(Flag::P));
    }
}
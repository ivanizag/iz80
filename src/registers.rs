use std::{fmt, mem, io};

/// 8 bit registers
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reg8 {
    /// 8 bit register A
    A = 0_u8,
    /// 8 bit register F, can be accessed with the flags methods
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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reg16 {
    /// 16 bit register AF
    AF = Reg8::A as u8,
    /// 16 bit register BC
    BC = Reg8::B as u8,
    /// 16 bit register DE
    DE = Reg8::D as u8,
    /// 16 bit register HL
    HL = Reg8::H as u8,
    /// 16 bit register IX
    IX = Reg8::IXH as u8,
    /// 16 bit register IY
    IY = Reg8::IYH as u8,
    /// 16 bit register SP
    SP = Reg8::SPH as u8
}

/// Z80 flags
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
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
            Reg8::_HL => write!(f, "(__index)"),
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
    im: u8,
    mode8080: bool
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
            im: 0,
            mode8080: false
        };

        reg.reset();
        reg
    }

    pub(crate) fn reset(&mut self) {
        self.set8(Reg8::I, 0x00);
        self.set8(Reg8::R, 0x00);
        self.set_interrupts(false);
        self.set_interrupt_mode(0);
        self.set16(Reg16::AF, 0xffff);
        self.set16(Reg16::SP, 0xffff);
        self.set_pc(0x0000);
    }

    pub(crate) fn set_8080(&mut self) {
        self.mode8080 = true;
        self.set16(Reg16::AF, 0xffff);
        self.set16(Reg16::SP, 0xffff);
        self.set_flag(Flag::N);
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
        assert!(!(reg == Reg8::_HL), "Can't use the pseudo register (HL)");
        self.data[reg as usize]
    }

    /// Sets the value of an 8 bit register
    #[inline]
    pub fn set8(&mut self, reg: Reg8, value: u8) {
        assert!(!(reg == Reg8::_HL), "Can't use the pseudo register (HL)");
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
        //if self.mode8080 && rr == Reg16::AF {
        if self.mode8080 && rr == Reg16::AF {
            // Ensure non existent flags have proper values
            self.set_flag(Flag::N);
            self.clear_flag(Flag::_3);
            self.clear_flag(Flag::_5);
        }
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
        mem::swap(&mut self.data[ih], &mut self.shadow[ih]);

        let il = rr as usize + 1;
        mem::swap(&mut self.data[il], &mut self.shadow[il]);
    }

    /// Returns the value of a flag
    #[inline]
    pub fn get_flag(&self, flag: Flag) -> bool {
        self.get8(Reg8::F) & flag as u8 != 0
    }

    /// Sets a flag. Sets the value to true
    #[inline]
    pub fn set_flag(&mut self, flag: Flag) {
        self.data[Reg8::F as usize] |= flag as u8;
    }

    /// Clears a flag. Sets the value to false
    #[inline]
    pub fn clear_flag(&mut self, flag: Flag) {
        self.data[Reg8::F as usize] &= !(flag as u8);
    }

    /// Sets the value of a flag
    #[inline]
    pub fn put_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }

    pub(crate) fn update_hn_flags(&mut self, hf: bool, nf: bool) {
        if !self.mode8080 {
            self.put_flag(Flag::H, hf);
            self.put_flag(Flag::N, nf);
        }
    }


    pub(crate) fn update_p_flag(&mut self, reference: u8) {
        let bits = reference.count_ones();
        self.put_flag(Flag::P, bits % 2 == 0);
    }

    pub(crate) fn update_sz53_flags(&mut self, reference: u8) {
        self.update_undocumented_flags(reference);
        self.put_flag(Flag::Z, reference == 0);
        self.put_flag(Flag::S, reference & (1<<7) != 0);
    }

    pub(crate) fn update_undocumented_flags(&mut self, reference: u8) {
        if !self.mode8080 {
            // Bits 5, and 3 are copied
            self.put_flag(Flag::_5, reference & (1<<5) != 0);
            self.put_flag(Flag::_3, reference & (1<<3) != 0);
        }
    }
    
    pub(crate) fn update_undocumented_flags_block(&mut self, reference: u8) {
        if !self.mode8080 {
            // TUZD-4.2
            self.put_flag(Flag::_5, reference & (1<<1) != 0);
            self.put_flag(Flag::_3, reference & (1<<3) != 0);
        }
    }

    pub(crate) fn update_add16_flags(&mut self, a: u32, b: u32, v: u32) {
        if self.mode8080 {
            self.put_flag(Flag::C, (v & 0x10000) != 0);
        } else {
            // TUZD-8.6
            // Flags are affected by the high order byte.
            // S, Z and P/V are not updated
            let xor = ((a ^ b ^ v) >> 8) as u16;
            self.update_undocumented_flags((v >> 8) as u8);
            self.put_flag(Flag::C, (xor >> 8 & 1) != 0);
            self.put_flag(Flag::H, (xor >> 4 & 1) != 0);
            self.clear_flag(Flag::N);
        }
    }

    pub(crate) fn update_arithmetic_flags_16(&mut self, a: u32, b: u32, reference: u32, neg: bool) {
        // No ADC or SBC on the 8080
        self.update_arithmetic_flags((a >> 8) as u16, (b >> 8) as u16, (reference >> 8) as u16, neg, true);
    }

    pub(crate) fn update_arithmetic_flags(&mut self, a: u16, b: u16, reference: u16, neg: bool, update_carry: bool) {
        self.update_sz53_flags(reference as u8);

        // TUZD-8.6
        let xor = a ^ b ^ reference;
        let carry_bit = (xor & 0x100) != 0;
        if update_carry {
            self.put_flag(Flag::C, carry_bit);
        }

        let half_bit  = (xor & 0x10) != 0;
        self.put_flag(Flag::H, half_bit);

        if self.mode8080 {
            self.update_p_flag(reference as u8);
            if neg {
                let a_b3 = (a & 0x08) != 0;
                let b_b3 = (b & 0x08) != 0;
                let r_b3 = (reference & 0x08) != 0;
                #[allow(clippy::nonminimal_bool)]
                let neg_half_bit = (!a_b3 && !b_b3 && !r_b3) || (a_b3 && !(b_b3 && r_b3)); 
                self.put_flag(Flag::H, neg_half_bit);
            }
        } else {
            let top_xor = (xor & 0x80) != 0;
            self.put_flag(Flag::P, carry_bit != top_xor); // As overflow flag
            self.put_flag(Flag::N, neg);
        }
    }

    pub(crate) fn update_logic_flags(&mut self, a: u8, b: u8, reference: u8, is_and: bool) {
        self.update_sz53_flags(reference);
        self.update_p_flag(reference);
        self.clear_flag(Flag::C);

        if self.mode8080 {
            self.put_flag(Flag::H, is_and && (((a | b) & 0x08) != 0));
        } else {
            self.clear_flag(Flag::N);
            self.put_flag(Flag::H, is_and);
        }
    }

    pub(crate) fn update_block_flags(&mut self, reference: u8, k: u16, counter: u8) {
        // TUZD-4.3
        self.update_sz53_flags(counter);

        self.put_flag(Flag::H, k>255);
        if !self.mode8080 {
            self.update_p_flag(k as u8 & 0x07 ^ counter);
            self.put_flag(Flag::N, reference & 0x80 != 0);
        }
        self.put_flag(Flag::C, k>255);
    }

    pub(crate) fn update_bits_in_flags(&mut self, reference: u8) {
        self.update_sz53_flags(reference);
        self.clear_flag(Flag::H);
        if !self.mode8080 {
            self.update_p_flag(reference);
            self.clear_flag(Flag::N);
        }
    }

    pub(crate) fn update_p_flag_with_iff2(&mut self) {
        self.put_flag(Flag::P, self.iff2);
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
        self.iff1 = v;
        self.iff2 = v;
    }

    pub(crate) fn set_interrupt_mode(&mut self, im: u8) {
        self.im = im;
    }

    pub(crate) fn get_interrupt_mode(&self) -> (bool, u8) {
        (self.iff1, self.im)
    }

    pub(crate) fn start_nmi(&mut self) {
        self.iff2 = self.iff1;
        self.iff1 = false;
    }

    pub(crate) fn end_nmi(&mut self) {
        self.iff1 = self.iff2;
    }

    pub(crate) const SERIALIZE_SIZE: usize = REG_COUNT8 + REG_COUNT8 + 5;

    pub(crate) fn serialize(&self) -> [u8; Registers::SERIALIZE_SIZE] {
        let mut data = [0; Registers::SERIALIZE_SIZE];
        let mut i = 0;
        for r in self.data.iter() {
            data[i] = *r;
            i += 1;
        }
        for r in self.shadow.iter() {
            data[i] = *r;
            i += 1;
        }
        data[i] = self.pc as u8;
        data[i+1] = (self.pc >> 8) as u8;
        data[i+2] = self.iff1 as u8;
        data[i+3] = self.iff2 as u8;
        data[i+4] = self.im;
        data
    }

    pub(crate) fn deserialize(&mut self, data: &[u8]) -> io::Result<()> {
        if data.len() < Registers::SERIALIZE_SIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Data too short"));
        }
        let mut i = 0;
        for r in self.data.iter_mut() {
            *r = data[i];
            i += 1;
        }
        for r in self.shadow.iter_mut() {
            *r = data[i];
            i += 1;
        }
        self.pc = u16::from_le_bytes([data[i], data[i+1]]);
        self.iff1 = data[i+2] != 0;
        self.iff2 = data[i+3] != 0;
        self.im = data[i+4];
        Ok(())
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
        assert!(r.get_flag(Flag::P));
        r.clear_flag(Flag::P);
        assert!(!r.get_flag(Flag::P));
        r.put_flag(Flag::P, true);
        assert!(r.get_flag(Flag::P));
        r.put_flag(Flag::P, false);
        assert!(!r.get_flag(Flag::P));
    }
}
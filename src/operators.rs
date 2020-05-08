use super::environment::*;
use super::registers::*;

pub type Operator = fn(&mut Environment, u8, u8) -> u8;

pub fn operator_add(env: &mut Environment, a: u8, b: u8) -> u8 {
    env.state.reg.clear_flag(Flag::C);
    operator_adc(env, a, b)
}

pub fn operator_adc(env: &mut Environment, a: u8, b: u8) -> u8 {
    let aa = a as u16;
    let bb = b as u16;
    let mut vv = aa + bb;
    if env.state.reg.get_flag(Flag::C) {
        vv += 1;
    }
    let v = vv as u8;

    //env.state.reg.update_arithmetic_flags(v, aa ^ bb ^ vv, false);
    env.state.reg.update_arithmetic_flags_alt(aa, bb, vv, false, true);

    v
}

pub fn operator_add16(env: &mut Environment, aa: u16, bb: u16) -> u16 {
    let aaaa = aa as u32;
    let bbbb = bb as u32;
    let vvvv = aaaa + bbbb;

    let vv = vvvv as u16;

    // TUZD-8.6
    // Flags are affected by the high order byte.
    // S, Z and P/V are not updated
    env.state.reg.update_undocumented_flags((vv >> 8) as u8);
    env.state.reg.update_ch_flags(((aaaa ^ bbbb ^ vvvv) >> 8) as u16);
    if !env.state.reg.mode8080 {
        env.state.reg.clear_flag(Flag::N);
    }
    vv
}

pub fn operator_adc16(env: &mut Environment, aa: u16, bb: u16) -> u16 {
    let aaaa = aa as u32;
    let bbbb = bb as u32;
    let mut vvvv = aaaa.wrapping_add(bbbb);
    if env.state.reg.get_flag(Flag::C) {
        vvvv = vvvv.wrapping_add(1);
    }
    let vv = vvvv as u16;

    // TUZD-8.6
    //env.state.reg.update_arithmetic_flags((vv >> 8) as u8, ((aaaa ^ bbbb ^ vvvv) >> 8) as u16, false);
    env.state.reg.update_arithmetic_flags_alt((aaaa >> 8) as u16, (bbbb >> 8) as u16, (vvvv >> 8) as u16, false, true);

    env.state.reg.put_flag(Flag::Z, vv == 0);

    vv
}

pub fn operator_sbc16(env: &mut Environment, aa: u16, bb: u16) -> u16 {
    let aaaa = aa as u32;
    let bbbb = bb as u32;
    let mut vvvv = aaaa.wrapping_sub(bbbb);
    if env.state.reg.get_flag(Flag::C) {
        vvvv = vvvv.wrapping_sub(1);
    }
    let vv = vvvv as u16;

    // TUZD-8.6
    //env.state.reg.update_arithmetic_flags((vv >> 8) as u8, ((aaaa ^ bbbb ^ vvvv) >> 8) as u16, true);
    env.state.reg.update_arithmetic_flags_alt((aaaa >> 8) as u16, (bbbb >> 8) as u16, (vvvv >> 8) as u16, true, true);
    env.state.reg.put_flag(Flag::Z, vv == 0);

    vv
}

pub fn operator_inc(env: &mut Environment, a: u8) -> u8 {
    let aa = a as u16;
    let vv = aa + 1;
    let v = vv as u8;

    //env.state.reg.update_inc_dec_flags(v, aa ^ vv, false);
    env.state.reg.update_arithmetic_flags_alt(aa, 0, vv, false, false);
    v
}

pub fn operator_sub(env: &mut Environment, a: u8, b: u8) -> u8 {
    env.state.reg.clear_flag(Flag::C);
    operator_sbc(env, a, b)
}

pub fn operator_sbc(env: &mut Environment, a: u8, b: u8) -> u8 {
    let aa = a as u16;
    let bb = b as u16;
    let mut vv = aa.wrapping_sub(bb);
    if env.state.reg.get_flag(Flag::C) {
        vv = vv.wrapping_sub(1);
    }

    let v = vv as u8;
    //env.state.reg.update_arithmetic_flags(v, aa ^ bb ^ vv, true);
    env.state.reg.update_arithmetic_flags_alt(aa, bb, vv, true, true);
    v
}

pub fn operator_dec(env: &mut Environment, a: u8) -> u8 {
    let aa = a as u16;
    let vv = aa.wrapping_sub(1);
    let v = vv as u8;
    //env.state.reg.update_inc_dec_flags(v, aa ^ vv, true);
    env.state.reg.update_arithmetic_flags_alt(aa, 0, vv, true, false);
    v
}

pub fn operator_and(env: &mut Environment, a: u8, b: u8) -> u8 {
    let v = a & b;
    env.state.reg.update_logic_flags(a, b, v, true);
    v
}

pub fn operator_xor(env: &mut Environment, a: u8, b: u8) -> u8 {
    let v = a ^ b;
    env.state.reg.update_logic_flags(a, b, v, false);
    v
}

pub fn operator_or(env: &mut Environment, a: u8, b: u8) -> u8 {
    let v = a | b;
    env.state.reg.update_logic_flags(a, b, v, false);
    v
}

pub fn operator_cp(env: &mut Environment, a: u8, b: u8) -> u8 {
    env.state.reg.clear_flag(Flag::C);
    operator_sub(env, a, b);

    // Note: flags 3 and 5 are taken from b. TUZD-8.4
    env.state.reg.update_undocumented_flags(b);
    a // Do not update the accumulator
}

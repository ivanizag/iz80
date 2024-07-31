use super::opcode::*;
use super::environment::*;
use super::registers::*;

// Relative jumps
pub fn build_djnz() -> Opcode {
    Opcode::new(
        "DJNZ d".to_string(),
        |env: &mut Environment| {
            let offset = env.advance_pc();
            let b = env.state.reg.get8(Reg8::B).wrapping_add(0xff /* -1 */);
            env.state.reg.set8(Reg8::B, b);
            if b != 0 {
                // Condition not met
                env.set_branch_taken();
                relative_jump(env, offset);
            }
        }
    )
}

pub fn build_jr_unconditional() -> Opcode {
    Opcode::new(
        "JR d".to_string(),
        |env: &mut Environment| {
            let offset = env.advance_pc();
            relative_jump(env, offset);
        }
    )
}

pub fn build_jr_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode::new(
        format!("JR {}, d", name),
        move |env: &mut Environment| {
            let offset = env.advance_pc();
            if env.state.reg.get_flag(flag) == value {
                env.set_branch_taken();
                relative_jump(env, offset);
            }
        }
    )
}


fn relative_jump(env: &mut Environment, offset: u8) {
    let mut pc = env.state.reg.pc();
    pc = pc.wrapping_add(offset as i8 as i16 as u16);
    env.state.reg.set_pc(pc);
}

// Absolute jumps
pub fn build_jp_unconditional() -> Opcode {
    Opcode::new(
        "JP nn".to_string(),
        |env: &mut Environment| {
            let address = env.advance_immediate16();
            env.state.reg.set_pc(address);
        }
    )
}

pub fn build_jp_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode::new(
        format!("JP {}, nn", name),
        move |env: &mut Environment| {
            let address = env.advance_immediate16();
            if env.state.reg.get_flag(flag) == value {
                env.set_branch_taken();
                env.state.reg.set_pc(address);
            }
        }
    )
}

pub fn build_jp_hl() -> Opcode {
    Opcode::new(
        "JP HL".to_string(), // Note: it is usaully written as JP (HL)
        |env: &mut Environment| {
            // Note: no displacement added to the index
            let address = env.index_value();
            env.state.reg.set_pc(address);
        }
    )
}

// Calls to subroutine
pub fn build_call() -> Opcode {
    Opcode::new(
        "CALL nn".to_string(),
        |env: &mut Environment| {
            let address = env.advance_immediate16();
            env.subroutine_call(address);
        }
    )
}

pub fn build_call_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode::new(
        format!("CALL {}, nn", name),
        move |env: &mut Environment| {
            let address = env.advance_immediate16();
            if env.state.reg.get_flag(flag) == value {
                env.set_branch_taken();
                env.subroutine_call(address);
            }
        }
    )
}

pub fn build_rst(d: u8) -> Opcode {
    Opcode::new(
        format!("RST {:02x}h", d),
        move |env: &mut Environment| {
            let address = d as u16;
            env.subroutine_call(address);
        }
    )
}

// Returns

pub fn build_ret() -> Opcode {
    Opcode::new(
        "RET".to_string(),
        |env: &mut Environment| {
            env.subroutine_return();
        }
    )
}

pub fn build_reti() -> Opcode {
    Opcode::new(
        "RETI".to_string(),
        |env: &mut Environment| {
            env.subroutine_return();
        }
    )
}

pub fn build_retn() -> Opcode {
    Opcode::new(
        "RETN".to_string(),
        |env: &mut Environment| {
            env.subroutine_return();
            env.state.reg.end_nmi();
        }
    )
}

pub fn build_ret_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode::new(
        format!("RET {}", name),
        move |env: &mut Environment| {
            if env.state.reg.get_flag(flag) == value {
                env.set_branch_taken();
                env.subroutine_return();
            }
        }
    )
}

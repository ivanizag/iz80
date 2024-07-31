use super::environment::*;
use super::registers::*;

type OpcodeFn = dyn Fn(&mut Environment);

pub struct Opcode {
    pub name: String,
    pub cycles: u8,
    pub cycles_conditional: u8,
    pub action: Box<OpcodeFn>,
}

impl Opcode {
    pub(crate) fn new<T>(name: String, action: T) -> Opcode
        where T: Fn(&mut Environment) + 'static {
        Opcode {
            name,
            cycles: 0,
            cycles_conditional: 0,
            action: Box::new(action),
        }
    }

    pub fn execute(&self, env: &mut Environment) {
        (self.action)(env);
    }

    pub fn disasm(&self, env: &Environment) -> String {
        let name = if self.name.contains("__index") {
            self.name.replace("__index", &env.index_description())
        } else {
            self.name.clone()
        };

        if self.name.contains("nn") {
            // Immediate argument 16 bits
            let nn = env.peek16_pc();
            let nn_str = format!("{:04x}h", nn);
            name.replace("nn", &nn_str)
        } else if self.name.contains('n') {
            // Immediate argument 8 bits
            let n = env.peek_pc();
            let n_str = format!("{:02x}h", n);
            name.replace('n', &n_str)
        } else if self.name.contains('d') {
            // Immediate argument 8 bits signed
            // In assembly it's shown with 2 added as if it were from the opcode pc.
            let d = env.peek_pc() as i8 as i16 + 2;
            let d_str = format!("{:+x}", d);
            name.replace('d', &d_str)
        } else {
            name
        }
    }
}

pub fn build_not_an_opcode() -> Opcode {
    Opcode::new(
        "NOT_AN_OPCODE".to_string(),
        |_: &mut Environment| {
            panic!("Not an opcode")
        }
    )
}

pub fn build_nop() -> Opcode {
    Opcode::new(
        "NOP".to_string(),
        |_: &mut Environment| {
            // Nothing done
        }
    )
}

pub fn build_noni_nop() -> Opcode {
    Opcode::new(
        "NONINOP".to_string(),
        |_: &mut Environment| {
            // Nothing done
        }
    )
}

pub fn build_halt() -> Opcode {
    Opcode::new(
        "HALT".to_string(),
        |env: &mut Environment| {
            env.state.halted = true;
        }
    )
}

pub fn build_pop_rr(rr: Reg16) -> Opcode {
    Opcode::new(
        format!("POP {rr:?}"),
        move |env: &mut Environment| {
            let value = env.pop();
            env.set_reg16(rr, value);
        }
    )
}

pub fn build_push_rr(rr: Reg16) -> Opcode {
    Opcode::new(
        format!("PUSH {rr:?}"),
        move |env: &mut Environment| {
            let value = env.reg16_ext(rr);
            env.push(value);
        }
    )
}

pub fn build_disable_interrupts() -> Opcode {
    Opcode::new(
        "DI".to_string(),
        |env: &mut Environment| {
            env.state.reg.set_interrupts(false);
        }
    )
}

pub fn build_enable_interrupts() -> Opcode {
    Opcode::new(
        "EI".to_string(),
        |env: &mut Environment| {
            env.state.reg.set_interrupts(true);
            env.state.int_just_enabled = true;
        }
    )
}

pub fn build_im(im: u8) -> Opcode {
    Opcode::new(
        format!("IM {im}"),
        move |env: &mut Environment| {
            env.state.reg.set_interrupt_mode(im);
        }
    )
}

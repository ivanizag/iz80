use super::environment::*;
use super::registers::*;

type OpcodeFn = dyn Fn(&mut Environment) -> ();

pub struct Opcode {
    pub name: String,
    pub action: Box<OpcodeFn>,
}

impl Opcode {
    pub fn execute(&self, env: &mut Environment) {
        (self.action)(env);
    }

    pub fn disasm(&self, env: &Environment) -> String {
        let name = format!("{} {}",
            env.index_description(), self.name);

        if self.name.contains("nn") {
            // Immediate argument 16 bits
            let nn = env.peek16_pc();
            let nn_str = format!("{:04x}h", nn);
            name.replace("nn", &nn_str)
        } else if self.name.contains("n") {
            // Immediate argument 8 bits
            let n = env.peek_pc();
            let n_str = format!("{:02x}h", n);
            name.replace("n", &n_str)
        } else if self.name.contains("d") {
            // Immediate argument 8 bits signed
            let d = env.peek_pc() as i8;
            let d_str = format!("{:+x}", d);
            name.replace("d", &d_str)
        } else {
            name
        }
    }
}

pub fn build_prefix(index: Reg16) -> Opcode {
    Opcode {
        name: format!("PREFIX {:?}", index),
        action: Box::new(move |env: &mut Environment| {
            // Change the index mode to IX or IY
            //let d = env.advance_pc() as i8;
            env.set_index(index /*, d*/);
        })
    }
}

pub fn build_nop() -> Opcode {
    Opcode {
        name: "NOP".to_string(),
        action: Box::new(|_: &mut Environment| {
            // Nothing done
        })
    }
}

pub fn build_noni_nop() -> Opcode {
    Opcode {
        name: "NONINOP".to_string(),
        action: Box::new(|_: &mut Environment| {
            // Nothing done
        })
    }
}

pub fn build_halt() -> Opcode {
    Opcode {
        name: "HALT".to_string(),
        action: Box::new(move |env: &mut Environment| {
            env.state.halted = true;
        })
    }
}

pub fn build_pop_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("POP {:?}", rr),
        action: Box::new(move |env: &mut Environment| {
            let value = env.pop();
            env.set_reg16(rr, value);
        })
    }
}

pub fn build_push_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("PUSH {:?}", rr),
        action: Box::new(move |env: &mut Environment| {
            let value = env.reg16_ext(rr);
            env.push(value);
        })
    }
}

pub fn build_conf_interrupts(enable: bool) -> Opcode {
    let name = if enable {"EI"} else  {"DI"};
    Opcode {
        name: name.to_string(),
        action: Box::new(move |env: &mut Environment| {
            env.state.reg.set_interrupts(enable);
        })
    }
}

pub fn build_im(im: u8) -> Opcode {
    Opcode {
        name: format!("IM {}", im),
        action: Box::new(move |env: &mut Environment| {
            env.state.reg.set_interrupt_mode(im);
        })
    }
}

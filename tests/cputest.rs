use iz80::*;

// Diagnostics II, version 1.2, CPU test by Supersoft Associates

static CODE: &[u8] = include_bytes!("res/CPUTEST.COM");

#[test]
fn test_cpu_test_8080() {
    cpu_test(Cpu::new_8080());
}

#[test]
fn test_cpu_test_z80() {
    cpu_test(Cpu::new_z80());
}

fn cpu_test(mut cpu: Cpu) {
    let mut machine = PlainMachine::new();

    // Load program
    let code = CODE;
    let size = code.len();
    for i in 0..size {
        machine.poke(0x100 + i as u16, code[i]);
    }

    /*
    System call 5

    .org $5
        ret
    */
    //let code = [0xD3, 0x00, 0xC9];
    let code = [0xC9];
    for i in 0..code.len() {
        machine.poke(5 + i as u16, code[i]);
    }

    cpu.registers().set_pc(0x100);
    let trace = false;
    cpu.set_trace(trace);
    let mut msg = String::new();
    loop {
        cpu.execute_instruction(&mut machine);

        // Avoid tracing the long loop
        if cpu.registers().pc() == 0x31b3 {
            cpu.set_trace(false);
        } else if cpu.registers().pc() == 0x31b5 {
            cpu.set_trace(trace);
        }

        if cpu.registers().pc() == 0x0000 {
            println!();
            break;
        }

        if cpu.registers().pc() == 0x0005 {
            match cpu.registers().get8(Reg8::C) {
                2 => {
                    // C_WRITE
                    let ch = cpu.registers().get8(Reg8::E) as char;
                    print!("{ch}");
                    msg.push(ch);
                },
                _ => panic!("BDOS command not implemented")
            }
        }
    }

    assert!(msg.contains("CPU TESTS OK"));
}
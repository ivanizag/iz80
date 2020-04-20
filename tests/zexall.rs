use iz80::*;

//static ZEXDOC: &'static [u8] = include_bytes!("res/zexdoc.com");
static ZEXALL: &'static [u8] = include_bytes!("res/zexall.com");

#[test]
#[ignore]
fn text_zexall() {
    let mut machine = ZexMachine::new();
    let mut cpu = Cpu::new();

    // Load program
    //let code = ZEXDOC;
    let code = ZEXALL;
    let size = code.len();
    for i in 0..size {
        machine.poke(0x100 + i as u16, code[i]);
    }

    /*
    System call 5

    .org $5
        out ($0), a
        ret
    */
    let code = [0xD3, 0x00, 0xC9];
    for i in 0..code.len() {
        machine.poke(5 + i as u16, code[i]);
    }

    // Patch to run a single test
    let run_single_test = false;
    let single_test = 11;
    if run_single_test {
        let mut test_start = machine.peek16(0x0120);
        test_start += single_test*2;
        machine.poke16(0x0120, test_start);
        machine.poke16(test_start + 2 , 0);
    
    }

    cpu.registers().set_pc(0x100);
    let trace = false;
    cpu.set_trace(trace);
    let mut tests_passed = 0;
    loop {
        cpu.execute_instruction(&mut machine);

        if trace {
            // Test state
            let addr = 0x1d80 as u16;
            print!("Zex state 0x{:04x}: ", addr);
            for i in 0..0x10 {
                print!("{:02x} ", machine.peek(addr + i));
            }
            println!("");
        }

        if cpu.registers().pc() == 0x0000 {
            println!("");
            break;
        }

        if machine.bdos_called {
            match cpu.registers().get8(Reg8::C) {
                2 => {
                    // C_WRITE
                    print!("{}", cpu.registers().get8(Reg8::E));
                },
                9 => {
                    // C_WRITE_STR
                    let mut address = cpu.registers().get16(Reg16::DE);
                    let mut msg = String::new();
                    loop {
                        let ch = machine.peek(address) as char;
                        address += 1;
                
                        if ch == '$'{
                            break;
                        }
                        msg.push(ch);
                    }
                    if msg.contains("OK") {
                        tests_passed += 1;
                    }
                    print!("{}", msg);
                },
                _ => panic!("BDOS command not implemented")
            }
            machine.bdos_called = false;
        }
    }

    if run_single_test {
        assert_eq!(1, tests_passed);
    } else {
        assert_eq!(67, tests_passed);
    }
}

struct ZexMachine {
    mem: [u8; 65536],
    bdos_called: bool
}

impl ZexMachine {
    pub fn new() -> ZexMachine {
        ZexMachine {
            mem: [0; 65536],
            bdos_called: false
        }
    }
}

impl Machine for ZexMachine {
    fn peek(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    fn poke(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    fn port_in(&mut self, _address: u16) -> u8 {
        0
    }

    fn port_out(&mut self, _address: u16, _value: u8) {
        self.bdos_called = true;
    }
}


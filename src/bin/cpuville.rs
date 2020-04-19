
/*
http://cpuville.com/Code/CPM-on-a-new-computer.html
http://cpuville.com/Code/Tiny-BASIC.html
*/
use std::io::*;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

use iz80::Cpu;
use iz80::Machine;
use iz80::State;

static TINY_BASIC: &'static [u8] = include_bytes!("rom/tinybasic2dms.bin");

fn main() {
    let mut machine = VilleMachine::new();
    let mut state = State::new();
    let cpu = Cpu::new();

    // Init console
    let mut stdout = stdout();
    let stdin_channel = spawn_stdin_channel();
    let mut in_char_waiting = false;

    // Load program
    let code = TINY_BASIC;
    let size = code.len();
    for i in 0..size {
        machine.poke(0x0000 + i as u16, code[i]);
    }

    // Init
    state.reg.set_pc(0x0000);
    machine.in_values[3] = 1; // TX Ready

    loop {
        cpu.execute_instruction(&mut state, &mut machine);

        if machine.out_called {
            match machine.out_port {
                2 => {
                    print!("{}", machine.out_value as char);
                    stdout.flush().unwrap();
                },
                3 => {},
                _ => panic!("BDOS command not implemented")
            }
            machine.out_called = false;
        }

        if machine.in_called {
            match machine.in_port {
                2 => {
                    in_char_waiting = false;
                },
                3 => {},
                _ => panic!("BDOS command not implemented")
            }
            machine.in_called = false;

            // Avoid 100% CPU usage waiting for input.
            thread::sleep(Duration::from_millis(1));  
        }

        if !in_char_waiting {
            // Let's get another char if available
            match stdin_channel.try_recv() {
                Ok(key) => {
                    machine.in_values[2] = key;
                    in_char_waiting = true;
                    machine.in_values[3] = 3; // RX Ready
                },
                Err(TryRecvError::Empty) => {
                    machine.in_values[3] = 1; // RX Not ready
                },
                Err(TryRecvError::Disconnected) => {},
            }
        }
    }
}

fn spawn_stdin_channel() -> Receiver<u8> {
    let (tx, rx) = mpsc::channel::<u8>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        for mut c in buffer.bytes() {
            if c == 10 {c = 13};
            tx.send(c).unwrap();
        }
    });
    rx
}

struct VilleMachine {
    mem: [u8; 65536],
    in_values: [u8; 256],
    in_called: bool,
    in_port: u8,
    out_called: bool,
    out_port: u8,
    out_value: u8
}

impl VilleMachine {
    pub fn new() -> VilleMachine {
        VilleMachine {
            mem: [0; 65536],
            in_values: [0; 256],
            out_called: false,
            out_port: 0,
            out_value: 0,
            in_called: false,
            in_port: 0
        }
    }
}

impl Machine for VilleMachine {
    fn peek(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    fn poke(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    fn port_in(&mut self, address: u16) -> u8 {
        let value = self.in_values[address as u8 as usize];
        if value != 1 {
            //print!("Port {:04x} in {:02x}\n", address, value);
        }
        self.in_port = address as u8;
        self.in_called = true;
        value
    }

    fn port_out(&mut self, address: u16, value: u8) {
        //print!("Port {:04x} out {:02x} {}\n", address, value, value as char);
        self.out_port = address as u8;
        self.out_value = value;
        self.out_called = true;
    }
}



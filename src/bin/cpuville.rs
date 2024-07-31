
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
use iz80::TimedRunner;

static TINY_BASIC: &[u8] = include_bytes!("rom/tinybasic2dms.bin");
const MHZ: f64 = 4.0;

fn main() {
    let mut machine = VilleMachine::new();
    let mut cpu = Cpu::new();
    let mut timed_runner = TimedRunner::default();
    timed_runner.set_mhz(&cpu, MHZ, 1000);

    // Init console
    let mut stdout = stdout();
    let stdin_channel = spawn_stdin_channel();
    let mut in_char_waiting = false;

    // Load program
    let code = TINY_BASIC;
    for (i, e) in code.iter().enumerate() {
        machine.poke(i as u16, *e);
    }

    // Init
    cpu.registers().set_pc(0x0000);
    machine.in_values[3] = 1; // TX Ready

    loop {
        timed_runner.execute(&mut cpu, &mut machine);

        if let Some(port) = machine.out_port {
            match port {
                2 => {
                    print!("{}", machine.out_value as char);
                    stdout.flush().unwrap();
                },
                3 => {},
                _ => panic!("BDOS command not implemented")
            }
            machine.out_port = None;
        }

        if let Some(port) = machine.in_port {
            match port {
                2 => {
                    in_char_waiting = false;
                },
                3 => {},
                _ => panic!("BDOS command not implemented")
            }
            machine.in_port = None;

            // Avoid 100% CPU usage waiting for input.
            if MHZ == 0.0 {
                thread::sleep(Duration::from_millis(1));  
            }
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
    in_port: Option<u8>,
    out_port: Option<u8>,
    out_value: u8
}

impl VilleMachine {
    pub fn new() -> VilleMachine {
        VilleMachine {
            mem: [0; 65536],
            in_values: [0; 256],
            out_port: None,
            out_value: 0,
            in_port: None
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
        self.in_port = Some(address as u8);
        value
    }

    fn port_out(&mut self, address: u16, value: u8) {
        //print!("Port {:04x} out {:02x} {}\n", address, value, value as char);
        self.out_port = Some(address as u8);
        self.out_value = value;
    }
}



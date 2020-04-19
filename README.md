# iz80

Z80 Emulator library for RUST. It passes all the tests of the ZEZALL suite. No cycle emulation accuracy, runs as fast as it can.

To run the ZEXALL test suite:
```
cargo test --release -- --nocapture --ignored
```

To run Tiny Basic (from [cpuville](http://cpuville.com/Kits/Z80-kits-home.html)):
```
cargo run --bin cpuville
```

## Usage
See [cpuville.rs](src/bin/cpuville.rs) or the CP/M 2.2 emulator [iz-cpm](https://github.com/ivanizag/iz-cpm) for more usage examples.

To run ths example, execute: `cargo run --bin simplest`
 
```
use iz80::*;

fn main() {
    // Prepare the device
    let mut machine = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();
    cpu.set_trace(true);

    // Load program inline or from a file with:
    //      let code = include_bytes!("XXXX.rom");
    let code = [0x3c, 0xc3, 0x00, 0x00]; // INC A, JP $0000
    let size = code.len();
    for i in 0..size {
        machine.poke(0x0000 + i as u16, code[i]);
    }

    // Run emulation
    state.reg.set_pc(0x0000);
    loop {
        cpu.execute_instruction(&mut state, &mut machine);

        // Examine Machine state to update the hosting device as needed.
        if state.reg.a() == 0x10 {
            // Let's stop
            break;
        }
    }
}



Runs Tiny Basic (version from [cpuville.com](http://cpuville.com/Code/Tiny-BASIC.html):
```
cargo run --bin cpuville
```

Passes the ZEXALL tests:
```
```



use iz80::*;

fn main() {
    // Prepare the device
    let mut machine = PlainMachine::new();
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
    cpu.registers().set_pc(0x0000);
    loop {
        cpu.execute_instruction(&mut machine);

        // Examine Machine state to update the hosting device as needed.
        if cpu.registers().a() == 0x10 {
            // Let's stop
            break;
        }
    }
}

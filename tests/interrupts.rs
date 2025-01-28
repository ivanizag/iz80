use iz80::*;

const IRQ_ADDRESS: u16 = 0x0038;

#[test]
fn test_ei() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new_z80();

    sys.poke(0x0000, 0xfb); // EI
    sys.poke(0x0001, 0xed); // IM 1
    sys.poke(0x0002, 0x56);

    cpu.execute_instruction(&mut sys);
    cpu.execute_instruction(&mut sys);
    cpu.execute_instruction(&mut sys);
    cpu.signal_interrupt(true);
    cpu.execute_instruction(&mut sys);

    assert_eq!(IRQ_ADDRESS+1, cpu.registers().pc());
}

#[test]
fn test_di() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new_z80();

    sys.poke(0x0000, 0xf3); // DI
    sys.poke(0x0001, 0xed); // IM 1
    sys.poke(0x0002, 0x56);

    cpu.execute_instruction(&mut sys);
    cpu.execute_instruction(&mut sys);
    cpu.execute_instruction(&mut sys);
    cpu.signal_interrupt(true);
    cpu.execute_instruction(&mut sys);

    assert_eq!(5, cpu.registers().pc());
}

#[test]
fn test_ei_on_handler() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new_z80();

    sys.poke(0x0000, 0xfb); // EI
    sys.poke(0x0001, 0xed); // IM 1
    sys.poke(0x0002, 0x56);

    sys.poke(IRQ_ADDRESS, 0x00); // NOP
    sys.poke(IRQ_ADDRESS+1, 0xfb); // EI
    sys.poke(IRQ_ADDRESS+2, 0xed); // RETI
    sys.poke(IRQ_ADDRESS+3, 0x4d);

    cpu.execute_instruction(&mut sys);
    cpu.execute_instruction(&mut sys);
    cpu.execute_instruction(&mut sys);
    cpu.signal_interrupt(true);

    cpu.execute_instruction(&mut sys);
    // On the handler, NOP executed
    assert_eq!(IRQ_ADDRESS+1, cpu.registers().pc());

    cpu.signal_interrupt(false);

    cpu.execute_instruction(&mut sys);
    // On the handler, EI executed

    cpu.execute_instruction(&mut sys);
    // RETI executed, even if interrupts are raised and enabled
    assert_eq!(4, cpu.registers().pc());

    cpu.execute_instruction(&mut sys);
    // INT not raised, executions continues
    assert_eq!(5, cpu.registers().pc());
}

#[test]
fn test_ei_on_handler_int_not_lowered() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new_z80();

    sys.poke(0x0000, 0xfb); // EI
    sys.poke(0x0001, 0xed); // IM 1
    sys.poke(0x0002, 0x56);

    sys.poke(IRQ_ADDRESS, 0x00); // NOP
    sys.poke(IRQ_ADDRESS+1, 0xfb); // EI
    sys.poke(IRQ_ADDRESS+2, 0xed); // RETI
    sys.poke(IRQ_ADDRESS+3, 0x4d);

    cpu.execute_instruction(&mut sys);
    cpu.execute_instruction(&mut sys);
    cpu.execute_instruction(&mut sys);
    cpu.signal_interrupt(true);

    cpu.execute_instruction(&mut sys);
    // On the handler, NOP executed
    assert_eq!(IRQ_ADDRESS+1, cpu.registers().pc());

    cpu.execute_instruction(&mut sys);
    // On the handler, EI executed

    cpu.execute_instruction(&mut sys);
    // RETI executed, even if interrupts are raised and enabled
    assert_eq!(4, cpu.registers().pc());

    cpu.execute_instruction(&mut sys);
    // On the handler again, as interrupts are raised and enabled
    assert_eq!(IRQ_ADDRESS+1, cpu.registers().pc());
}

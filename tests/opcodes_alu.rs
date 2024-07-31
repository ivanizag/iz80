use iz80::*;

#[test]
fn test_cp_a() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new_8080();

    sys.poke(0x0000, 0xfe); // CP A, 01h
    sys.poke(0x0001, 0x01);
    cpu.registers().set_a(0x10);
    cpu.registers().clear_flag(Flag::H);

    cpu.execute_instruction(&mut sys);

    assert!(!cpu.registers().get_flag(Flag::H));
}

#[test]
fn test_cp_a_2() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new_8080();

    sys.poke(0x0000, 0xfe); // CP A, 01h
    sys.poke(0x0001, 0x01);
    cpu.registers().set_a(0x08);
    cpu.registers().clear_flag(Flag::H);

    cpu.execute_instruction(&mut sys);

    assert!(cpu.registers().get_flag(Flag::H));
}

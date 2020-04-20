use iz80::*;

#[test]
fn test_out_e() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xed); // OUT (C), E
    sys.poke(0x0001, 0x59);
    cpu.registers().set8(Reg8::E, 0x63);
    cpu.registers().set16(Reg16::BC, 0x6345);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0x63, sys.port_in(0x6345));
}

#[test]
fn test_in_e() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xed); // IN E, (C)
    sys.poke(0x0001, 0x58);
    cpu.registers().set16(Reg16::BC, 0x6345);
    sys.port_out(0x6345, 0x8a);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0x8a, sys.port_in(0x6345));
}

use iz80::*;

#[test]
fn test_neg_a() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xed);  // NEG
    sys.poke(0x0001, 0x44);
    cpu.registers().set_a(0xff);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0x01, cpu.registers().a());
}

#[test]
fn test_inc_a() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x3c);  // INC A
    cpu.registers().set_a(0xa4);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0xa5, cpu.registers().a());
}

#[test]
fn test_inc_a_overflow() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x3c);  // INC A
    cpu.registers().set_a(0xff);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0x00, cpu.registers().a());
}

#[test]
fn test_inc_e() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x1c);  // INC E
    cpu.registers().set8(Reg8::E, 0x14);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0x15, cpu.registers().get8(Reg8::E));
}

#[test]
fn test_dec_a() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x3d);  // DEC A
    cpu.registers().set_a(0xa4);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0xa3, cpu.registers().a());
}

#[test]
fn test_dec_a_underflow() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x3d);  // DEC A
    cpu.registers().set_a(0x00);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0xff, cpu.registers().a());
}

#[test]
fn test_inc_de() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x13);  // INC DE
    cpu.registers().set16(Reg16::DE, 0xcea4);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0xcea5, cpu.registers().get16(Reg16::DE));
}

#[test]
fn test_inc_de_overflow() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x13);  // INC DE
    cpu.registers().set16(Reg16::DE, 0xffff);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0x0000, cpu.registers().get16(Reg16::DE));
}

#[test]
fn test_dec_de() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x1b);  // DEC A
    cpu.registers().set16(Reg16::DE, 0x1256);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0x1255, cpu.registers().get16(Reg16::DE));
}

#[test]
fn test_dec_de_underflow() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x1b);  // DEC DE
    cpu.registers().set16(Reg16::DE, 0x0000);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0xffff, cpu.registers().get16(Reg16::DE));
}

#[test]
fn test_dec_phl() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x35);  // DEC (HL)
    cpu.registers().set16(Reg16::HL, 0x23c4);
    sys.poke(0x23c4, 0x67);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0x66, sys.peek(0x23c4));
}

#[test]
fn test_add_hl_de() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x19);  // ADD HL, DE
    cpu.registers().set16(Reg16::HL, 0x1234);
    cpu.registers().set16(Reg16::DE, 0x0101);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0x1335, cpu.registers().get16(Reg16::HL));
}

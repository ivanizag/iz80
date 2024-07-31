use iz80::*;

#[test]
fn test_rrca_fast() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x0f); // RRCA
    cpu.registers().set_a(0b1001_0011);
    cpu.registers().set_flag(Flag::C);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0b1100_1001, cpu.registers().a());
    assert!(cpu.registers().get_flag(Flag::C));
}

#[test]
fn test_rrc_a() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // RRC A
    sys.poke(0x0001, 0x0f);
    cpu.registers().set_a(0b1001_0011);
    cpu.registers().set_flag(Flag::C);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0b1100_1001, cpu.registers().a());
    assert!(cpu.registers().get_flag(Flag::C));
}

#[test]
fn test_rr_b() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // RR B
    sys.poke(0x0001, 0x18);
    cpu.registers().set8(Reg8::B, 0b1001_0010);
    cpu.registers().set_flag(Flag::C);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0b1100_1001, cpu.registers().get8(Reg8::B));
    assert!(!cpu.registers().get_flag(Flag::C));
}

#[test]
fn test_sra_c() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // SRA C
    sys.poke(0x0001, 0x29);
    cpu.registers().set8(Reg8::C, 0b1001_0011);
    cpu.registers().clear_flag(Flag::C);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0b1100_1001, cpu.registers().get8(Reg8::C));
    assert!(cpu.registers().get_flag(Flag::C));
}

#[test]
fn test_srl_d() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // SRL D
    sys.poke(0x0001, 0x3a);
    cpu.registers().set8(Reg8::D, 0b1001_0011);
    cpu.registers().clear_flag(Flag::C);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0b0100_1001, cpu.registers().get8(Reg8::D));
    assert!(cpu.registers().get_flag(Flag::C));
}

#[test]
fn test_rlc_a() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // RLC A
    sys.poke(0x0001, 0x07);
    cpu.registers().set_a(0b0001_0011);
    cpu.registers().set_flag(Flag::C);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0b0010_0110, cpu.registers().a());
    assert!(!cpu.registers().get_flag(Flag::C));
}

#[test]
fn test_rl_b() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // RL B
    sys.poke(0x0001, 0x10);
    cpu.registers().set8(Reg8::B, 0b0001_0011);
    cpu.registers().set_flag(Flag::C);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0b0010_0111, cpu.registers().get8(Reg8::B));
    assert!(!cpu.registers().get_flag(Flag::C));
}

#[test]
fn test_sla_c() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // SLA C
    sys.poke(0x0001, 0x21);
    cpu.registers().set8(Reg8::C, 0b1001_0011);
    cpu.registers().clear_flag(Flag::C);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0b0010_0110, cpu.registers().get8(Reg8::C));
    assert!(cpu.registers().get_flag(Flag::C));
}

#[test]
fn test_sll_d() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // SLL D
    sys.poke(0x0001, 0x32);
    cpu.registers().set8(Reg8::D, 0b1001_0011);
    cpu.registers().clear_flag(Flag::C);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0b0010_0111, cpu.registers().get8(Reg8::D));
    assert!(cpu.registers().get_flag(Flag::C));
}

#[test]
fn test_bit_a() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // BIT 1, A
    sys.poke(0x0001, 0x4f);
    cpu.registers().set_a(0b0001_0010);
    cpu.registers().set_flag(Flag::Z);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0b0001_0010, cpu.registers().a());
    assert!(!cpu.registers().get_flag(Flag::Z));
}

#[test]
fn test_set_b() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // SET 0, B
    sys.poke(0x0001, 0xc0);
    cpu.registers().set8(Reg8::B, 0b0001_0010);
    cpu.registers().clear_flag(Flag::Z);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0b0001_0011, cpu.registers().get8(Reg8::B));
    assert!(!cpu.registers().get_flag(Flag::Z));
}

#[test]
fn test_res_c() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // RES 7, C
    sys.poke(0x0001, 0xb9);
    cpu.registers().set8(Reg8::C, 0b1001_0011);
    cpu.registers().clear_flag(Flag::Z);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0b0001_0011, cpu.registers().get8(Reg8::C));
    assert!(!cpu.registers().get_flag(Flag::Z));
}

#[test]
fn test_cpl() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x2f);  // CPL
    cpu.registers().set_a(0x3d);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0xc2, cpu.registers().a());
}

#[test]
fn test_rld() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xed); // RLD
    sys.poke(0x0001, 0x6f);
    cpu.registers().set_a(0xab);
    cpu.registers().set16(Reg16::HL, 0xccdd);
    sys.poke(0xccdd, 0xcd);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0xac, cpu.registers().a());
    assert_eq!(0xdb, sys.peek(0xccdd));
}

#[test]
fn test_rrd() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xed); // RRD
    sys.poke(0x0001, 0x67);
    cpu.registers().set_a(0xab);
    cpu.registers().set16(Reg16::HL, 0xccdd);
    sys.poke(0xccdd, 0xcd);

    cpu.execute_instruction(&mut sys);

    assert_eq!(0xad, cpu.registers().a());
    assert_eq!(0xbc, sys.peek(0xccdd));
}
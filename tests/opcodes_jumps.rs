use iz80::*;

#[test]
fn test_djnz_jump() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x10);  // DJNZ +$06
    sys.poke(0x0001, 0x06); 
    cpu.registers().set8(Reg8::B, 0x23);

    cpu.execute_instruction(&mut sys);
    assert_eq!(0x22, cpu.registers().get8(Reg8::B));
    assert_eq!(0x0006, cpu.registers().pc());
}

#[test]
fn test_djnz_no_jump() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x10);  // DJNZ +$06
    sys.poke(0x0001, 0x06); 
    cpu.registers().set8(Reg8::B, 0x01);

    cpu.execute_instruction(&mut sys);
    assert_eq!(0x00, cpu.registers().get8(Reg8::B));
    assert_eq!(0x0002, cpu.registers().pc());
}

#[test]
fn test_jr_z_jump() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x10);  // JR -$02
    sys.poke(0x0001, 0xfe); 
    cpu.registers().set_flag(Flag::Z);

    cpu.execute_instruction(&mut sys);
    assert_eq!(0xFFFE, cpu.registers().pc());
}

#[test]
fn test_jp() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xc3);  // JP $2000
    sys.poke(0x0001, 0x00); 
    sys.poke(0x0002, 0x20);
    
    cpu.execute_instruction(&mut sys);
    assert_eq!(0x2000, cpu.registers().pc());
}

#[test]
fn test_call() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcd);  // CALL $2000
    sys.poke(0x0001, 0x00); 
    sys.poke(0x0002, 0x20);
    
 
    cpu.execute_instruction(&mut sys);
    assert_eq!(0x2000, cpu.registers().pc());
    //assert_eq!(0x0003, cpu.env.pop());
}

#[test]
fn test_call_z_jump() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcc);  // CALL Z $2000
    sys.poke(0x0001, 0x00); 
    sys.poke(0x0002, 0x20);
    cpu.registers().set_flag(Flag::Z);
     
    cpu.execute_instruction(&mut sys);
    assert_eq!(0x2000, cpu.registers().pc());
    //assert_eq!(0x0003, cpu.env.pop());
}

#[test]
fn test_call_z_no_jump() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcc);  // CALL Z $2000
    sys.poke(0x0001, 0x00); 
    sys.poke(0x0002, 0x20);
    cpu.registers().clear_flag(Flag::Z);
     
    cpu.execute_instruction(&mut sys);
    assert_eq!(0x0003, cpu.registers().pc());
}

#[test]
fn test_rst() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xff);  // RST 38h    
 
    cpu.execute_instruction(&mut sys);
    assert_eq!(0x0038, cpu.registers().pc());
    //assert_eq!(0x0001, cpu.env.pop());
}

#[test]
fn test_call_ret() {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcd);  // CALL $2000
    sys.poke(0x0001, 0x00); 
    sys.poke(0x0002, 0x20);

    sys.poke(0x2000, 0xc9);  // RET
    
    cpu.execute_instruction(&mut sys);
    assert_eq!(0x2000, cpu.registers().pc());
     cpu.execute_instruction(&mut sys);
    assert_eq!(0x0003, cpu.registers().pc());
}

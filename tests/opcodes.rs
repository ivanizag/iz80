use z80::cpu::Cpu;
use z80::state::State;
use z80::machine::*;
use z80::registers::*;

#[test]
fn test_two_opcodes() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x06);  // LD B, $34
    sys.poke(0x0001, 0x34);
    sys.poke(0x0002, 0x78);  // LD A, B
 
    cpu.execute_instruction(&mut state, &mut sys);
    cpu.execute_instruction(&mut state, &mut sys);

    println!("Registers: {:?}", state.reg);

    assert_eq!(0x34, state.reg.a());
}

#[test]
fn test_push_pop_rr() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xc5);  // PUSH BC
    sys.poke(0x0001, 0xf1);  // POP AF
    state.reg.set16(Reg16::AF, 0x5678);
    state.reg.set16(Reg16::BC, 0x1234);

    cpu.execute_instruction(&mut state, &mut sys);
    assert_eq!(0x1234, state.reg.get16(Reg16::BC));
    assert_eq!(0x5678, state.reg.get16(Reg16::AF));

    cpu.execute_instruction(&mut state, &mut sys);
    assert_eq!(0x1234, state.reg.get16(Reg16::BC));
    assert_eq!(0x1234, state.reg.get16(Reg16::AF));
}

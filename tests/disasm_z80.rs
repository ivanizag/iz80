use iz80::*;

fn test_disasm_z80(code: &[u8], expected: &str) {
    let mut sys = PlainMachine::new();
    let mut cpu = Cpu::new();

    for i in 0..code.len() {
        sys.poke(i as u16, code[i]);
    }

    let disasm = cpu.disasm_instruction(&mut sys);
    assert_eq!(expected, disasm);
}

#[test]
fn test_disasm_nop() {
    test_disasm_z80(&[0x00], "NOP");
}

#[test]
fn test_disasm_ld_hl_n() {
    test_disasm_z80(&[0x36, 0x33], "LD (HL), 33h");
}

#[test]
fn test_disasm_ld_ix_d_n() {
    test_disasm_z80(&[0xdd, 0x36, 22, 0x33], "LD (IX+22), 33h");
}

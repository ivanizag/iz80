use iz80::*;

#[test]
fn test_serialization_deserialization() {
    let mut cpu = Cpu::new();

    cpu.registers().set8(Reg8::A, 0x12);
    cpu.registers().set8(Reg8::F, 0x23);
    cpu.registers().set8(Reg8::B, 0x34);
    cpu.registers().set8(Reg8::C, 0x56);
    cpu.registers().set8(Reg8::D, 0x78);
    cpu.registers().set8(Reg8::E, 0x9a);
    cpu.registers().set8(Reg8::H, 0xbc);
    cpu.registers().set8(Reg8::L, 0xde);
    cpu.registers().set8(Reg8::I, 0xf0);
    cpu.registers().set8(Reg8::R, 0x12);
    cpu.registers().set8(Reg8::IXH, 0x34);
    cpu.registers().set8(Reg8::IXL, 0x56);
    cpu.registers().set8(Reg8::IYH, 0x78);
    cpu.registers().set8(Reg8::IYL, 0x9a);
    cpu.registers().set8(Reg8::SPH, 0xbc);
    cpu.registers().set8(Reg8::SPL, 0xde);
    cpu.registers().set_pc(0xabcd);


    let serialized = cpu.serialize();
    let mut cpu2 = Cpu::new();
    let result = cpu2.deserialize(&serialized);

    assert!(result.is_ok());

    assert_eq!(0x12, cpu2.registers().get8(Reg8::A), "Bad serialization of register A");
    assert_eq!(0x23, cpu2.registers().get8(Reg8::F), "Bad serialization of register F");
    assert_eq!(0x34, cpu2.registers().get8(Reg8::B), "Bad serialization of register B");
    assert_eq!(0x56, cpu2.registers().get8(Reg8::C), "Bad serialization of register C");
    assert_eq!(0x78, cpu2.registers().get8(Reg8::D), "Bad serialization of register D");
    assert_eq!(0x9a, cpu2.registers().get8(Reg8::E), "Bad serialization of register E");
    assert_eq!(0xbc, cpu2.registers().get8(Reg8::H), "Bad serialization of register H");
    assert_eq!(0xde, cpu2.registers().get8(Reg8::L), "Bad serialization of register L");
    assert_eq!(0xf0, cpu2.registers().get8(Reg8::I), "Bad serialization of register I");
    assert_eq!(0x12, cpu2.registers().get8(Reg8::R), "Bad serialization of register R");
    assert_eq!(0x34, cpu2.registers().get8(Reg8::IXH), "Bad serialization of register IXH");
    assert_eq!(0x56, cpu2.registers().get8(Reg8::IXL), "Bad serialization of register IXL");
    assert_eq!(0x78, cpu2.registers().get8(Reg8::IYH), "Bad serialization of register IYH");
    assert_eq!(0x9a, cpu2.registers().get8(Reg8::IYL), "Bad serialization of register IYL");
    assert_eq!(0xbc, cpu2.registers().get8(Reg8::SPH), "Bad serialization of register SPH");
    assert_eq!(0xde, cpu2.registers().get8(Reg8::SPL), "Bad serialization of register SPL");
    assert_eq!(0xabcd, cpu2.registers().pc(), "Bad serialization of register PC");
 
}

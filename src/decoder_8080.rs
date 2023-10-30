use super::cpu::*;
use super::opcode::*;
use super::opcode_alu::*;
use super::opcode_arith::*;
use super::opcode_io::*;
use super::opcode_bits::*;
use super::opcode_jumps::*;
use super::opcode_ld::*;
use super::operators::*;
use super::registers::*;
use super::environment::*;

/* See
    http://www.z80.info/decoding.htm
    http://clrhome.org/table/
    http://z80-heaven.wikidot.com/instructions-set
*/

pub struct Decoder8080 {
    no_prefix: [Option<Opcode>; 256],
}

impl Decoder for Decoder8080 {
    fn decode(&self, env: &mut Environment) -> &Opcode {
        let b0 = env.advance_pc();
        let opcode = &self.no_prefix[b0 as usize];
        match opcode {
            Some(o) => o,
            None => {
                panic!("Opcode {:02x} not defined", b0);
            }
        }
    }
}

impl Decoder8080 {
    pub fn new() -> Decoder8080 {

        let mut decoder = Decoder8080 {
            no_prefix: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            ]
        };
        decoder.load_no_prefix();
        decoder.load_cycle_information();
        decoder
    }

    fn load_no_prefix(&mut self) {
        for c in 0..=255 {
            let p = DecodingHelper::parts(c);
            let opcode = match p.x {
                0 => match p.z {
                    0 => Some(build_nop()), // NOP
                    1 => match p.q {
                        0 =>  Some(build_ld_rr_nn(RP[p.p])), // LD rr, nn -- 16-bit load add
                        1 =>  Some(build_add_hl_rr(RP[p.p])), // ADD HL, rr -- 16-bit add
                        _ => panic!("Unreachable")
                    },
                    2 => match p.q {
                        0 =>  match p.p {
                            0 => Some(build_ld_prr_a(Reg16::BC)), // LD (BC), A
                            1 => Some(build_ld_prr_a(Reg16::DE)), // LD (DE), A
                            2 => Some(build_ld_pnn_rr(Reg16::HL, true)), // LD (nn), HL
                            3 => Some(build_ld_pnn_a()), // LD (nn), A
                            _ => panic!("Unreachable")
                        },
                        1 =>  match p.p {
                            0 => Some(build_ld_a_prr(Reg16::BC)), // LD A, (BC)
                            1 => Some(build_ld_a_prr(Reg16::DE)), // LD A, (DE)
                            2 => Some(build_ld_rr_pnn(Reg16::HL, true)), // LD HL, (nn)
                            3 => Some(build_ld_a_pnn()), // LD A, (nn)
                            _ => panic!("Unreachable")
                        }
                        _ => panic!("Unreachable")
                    },
                    3 => match p.q {
                        0 =>  Some(build_inc_dec_rr(RP[p.p], true)), // INC rr -- 16-bit inc
                        1 =>  Some(build_inc_dec_rr(RP[p.p], false)), // DEC rr -- 16-bit dec
                        _ => panic!("Unreachable")                       
                    },
                    4 => Some(build_inc_r(R[p.y])), // INC r -- 8 bit inc
                    5 => Some(build_dec_r(R[p.y])), // DEC r -- 8 bit dec
                    6 => Some(build_ld_r_n(R[p.y])), // LD r, n -- 8 bit load imm
                    7 => match p.y {
                        0..=3 => Some(build_rot_r(Reg8::A, ROT[p.y], true, false)), // rotA
                        4 => Some(build_daa8080()), // DAA, decimal adjust A
                        5 => Some(build_cpl()), // CPL, complement adjust A
                        6 => Some(build_scf()), // SCF, set carry flag
                        7 => Some(build_ccf()), // CCF, clear carry flag
                        _ => panic!("Unreachable")
                    },
                    _ => panic!("Unreachable")
                },
                1 => match (p.z, p.y) {
                    (6, 6) => Some(build_halt()), // HALT, exception instead of LD (HL), (HL)
                    _ => Some(build_ld_r_r(R[p.y], R[p.z], false)), // LD r[y], r[z] -- 8 bit load imm
                },
            2 => Some(build_operator_a_r(R[p.z], ALU[p.y])), // alu A, r
            3 => match p.z {
                    0 => Some(build_ret_eq(CC[p.y])), // RET cc
                    1 => match p.q {
                        0 => Some(build_pop_rr(RP2[p.p])), // POP rr
                        1 => match p.p {
                            0 | 1 => Some(build_ret()), // RET
                            2 => Some(build_jp_hl()), // JP HL
                            3 => Some(build_ld_sp_hl()), // LD SP, HL
                            _ => panic!("Unreachable")
                        },
                        _ => panic!("Unreachable")
                    },
                    2 => Some(build_jp_eq(CC[p.y])), // JP cc, nn
                    3 => match p.y {
                        0 | 1 => Some(build_jp_unconditional()), // JP nn
                        2 => Some(build_out_n_a()),  // OUT (n), A
                        3 => Some(build_in_a_n()),   // IN A, (n)
                        4 => Some(build_ex_psp_hl()), // EX (SP), HL
                        5 => Some(build_ex_de_hl()),  // EX DE, HL
                        6 => Some(build_disable_interrupts()), // DI
                        7 => Some(build_enable_interrupts()),  // EI
                        _ => panic!("Unreachable")
                    }
                    4 => Some(build_call_eq(CC[p.y])),
                    5 => match p.q {
                        0 => Some(build_push_rr(RP2[p.p])), // PUSH rr
                        1 => Some(build_call()), // Call nn
                        _ => panic!("Unreachable")
                    },
                    6 => Some(build_operator_a_n(ALU[p.y])), // alu A, n
                    7 => Some(build_rst(p.y as u8 * 8)), // RST
                    _ => panic!("Unreachable")
                    },
                _ => panic!("Unreachable")
            };

            /*
            match opcode.as_ref() {
                None => println!("0x{:02x} {:20}: {:?}", c, "Pending", p),
                Some(o) => println!("0x{:02x} {:20}: {:?}", c, o.name, p)
            }
            */

            self.no_prefix[c as usize] = opcode;
        }
    }

    fn load_cycle_information(&mut self) {

        // Load cycle information
        for c in 0..=255 {
            if let Some(opcode) = &mut self.no_prefix[c as usize] {
                opcode.cycles = NO_PREFIX_CYCLES[c];
                opcode.cycles_conditional = opcode.cycles;
            }
        }

        //Load cycle information for conditional cases
        if let Some(opcode) = &mut self.no_prefix[0xc0] { opcode.cycles_conditional =  5; }
        if let Some(opcode) = &mut self.no_prefix[0xc4] { opcode.cycles_conditional = 11; }
        if let Some(opcode) = &mut self.no_prefix[0xc8] { opcode.cycles_conditional =  5; }
        if let Some(opcode) = &mut self.no_prefix[0xcc] { opcode.cycles_conditional = 11; }

        if let Some(opcode) = &mut self.no_prefix[0xd0] { opcode.cycles_conditional =  5; }
        if let Some(opcode) = &mut self.no_prefix[0xd4] { opcode.cycles_conditional = 11; }
        if let Some(opcode) = &mut self.no_prefix[0xd8] { opcode.cycles_conditional =  5; }
        if let Some(opcode) = &mut self.no_prefix[0xdc] { opcode.cycles_conditional = 11; }

        if let Some(opcode) = &mut self.no_prefix[0xe0] { opcode.cycles_conditional =  5; }
        if let Some(opcode) = &mut self.no_prefix[0xe4] { opcode.cycles_conditional = 11; }
        if let Some(opcode) = &mut self.no_prefix[0xe8] { opcode.cycles_conditional =  5; }
        if let Some(opcode) = &mut self.no_prefix[0xec] { opcode.cycles_conditional = 11; }

        if let Some(opcode) = &mut self.no_prefix[0xf0] { opcode.cycles_conditional =  5; }
        if let Some(opcode) = &mut self.no_prefix[0xf4] { opcode.cycles_conditional = 11; }
        if let Some(opcode) = &mut self.no_prefix[0xf8] { opcode.cycles_conditional =  5; }
        if let Some(opcode) = &mut self.no_prefix[0xfc] { opcode.cycles_conditional = 11; }
        // Note that on the 8080 the conditional jumps use always the same number of cycles
    }
}

#[derive(Debug)]
struct DecodingHelper {
    // See notation in http://www.z80.info/decoding.htm    
    x: usize,
    y: usize,
    z: usize,
    p: usize,
    q: usize
}

impl DecodingHelper {
    fn parts(code: u8) -> DecodingHelper {
        DecodingHelper {
            x: (code >> 6) as usize,
            y: ((code >> 3) & 7) as usize,
            z: (code & 7) as usize,
            p: ((code >> 4) & 3) as usize,
            q: ((code >> 3) & 1) as usize,
        }
    }
}

const RP:  [Reg16; 4] = [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::SP];
const RP2: [Reg16; 4] = [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::AF];
const R:  [Reg8; 8] = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::_HL, Reg8::A];

const CC: [(Flag, bool, &str); 8] = [
    (Flag::Z, false, "NZ"),
    (Flag::Z, true,  "Z"),
    (Flag::C, false, "NC"),
    (Flag::C, true,  "C"),
    (Flag::P, false, "PO"),
    (Flag::P, true,  "PE"),
    (Flag::S, false, "P"),
    (Flag::S, true,  "N")
];

const ROT: [(ShiftDir, ShiftMode, &str); 8] = [
    (ShiftDir::Left,  ShiftMode::RotateCarry, "RLC"),
    (ShiftDir::Right, ShiftMode::RotateCarry, "RRC"),
    (ShiftDir::Left,  ShiftMode::Rotate,      "RL" ),
    (ShiftDir::Right, ShiftMode::Rotate,      "RR" ),
    (ShiftDir::Left,  ShiftMode::Arithmetic,  "SLA"),
    (ShiftDir::Right, ShiftMode::Arithmetic,  "SRA"),
    (ShiftDir::Left,  ShiftMode::Logical,     "SLL"),
    (ShiftDir::Right, ShiftMode::Logical,     "SRL"),
];

const ALU: [(Operator, &str); 8] = [
    (operator_add, "ADD"),
    (operator_adc, "ADC"),
    (operator_sub, "SUB"),
    (operator_sbc, "SBC"),
    (operator_and, "AND"),
    (operator_xor, "XOR"),
    (operator_or,  "OR"),
    (operator_cp,  "CP")
];

// From https://pastraiser.com/cpu/i8080/i8080_opcodes.html
// From https://tobiasvl.github.io/optable/intel-8080/
const NO_PREFIX_CYCLES: [u8; 256] = [
     4, 10,  7,  5,  5,  5,  7,  4,  4, 10,  7,  5,  5,  5,  7,  4,
     4, 10,  7,  5,  5,  5,  7,  4,  4, 10,  7,  5,  5,  5,  7,  4,
     4, 10, 16,  5,  5,  5,  7,  4,  4, 10, 16,  5,  5,  5,  7,  4,
     4, 10, 13,  5, 10, 10, 10,  4,  4, 10, 13,  5,  5,  5,  7,  4,

     5,  5,  5,  5,  5,  5,  7,  5,  5,  5,  5,  5,  5,  5,  7,  5,
     5,  5,  5,  5,  5,  5,  7,  5,  5,  5,  5,  5,  5,  5,  7,  5,
     5,  5,  5,  5,  5,  5,  7,  5,  5,  5,  5,  5,  5,  5,  7,  5,
     7,  7,  7,  7,  7,  7,  7,  7,  5,  5,  5,  5,  5,  5,  7,  5,

     4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,
     4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,
     4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,
     7,  7,  7,  7,  7,  7,  4,  7,  4,  4,  4,  4,  4,  4,  7,  4,

    11, 10, 10, 10, 17, 11,  7, 11, 11, 10, 10, 10, 17, 17,  7, 11,
    11, 10, 10, 10, 17, 11,  7, 11, 11, 10, 10, 10, 17, 17,  7, 11,
    11, 10, 10, 18, 17, 11,  7, 11, 11,  5, 10,  5, 17, 17,  7, 11,
    11, 10, 10,  4, 17, 11,  7, 11, 11,  5, 10,  4, 17, 17,  7, 11,
];
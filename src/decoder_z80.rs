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

pub struct DecoderZ80 {
    no_prefix: [Opcode; 256],
    prefix_cb: [Opcode; 256],
    prefix_cb_indexed: [Opcode; 256],
    prefix_ed: [Opcode; 256],
    has_displacement: [bool; 256],
}

impl Decoder for DecoderZ80 {
    fn decode(&self, env: &mut Environment) -> &Opcode {
        let mut code = env.advance_pc();

        // Process prefixes even if reapeated
        while code == 0xdd || code == 0xfd {
            if code == 0xdd {
                // DD prefix
                env.set_index(Reg16::IX);
                code = env.advance_pc();
            } else {
                // FD prefix
                env.set_index(Reg16::IY);
                code = env.advance_pc();
            }
        }
        
        match code {
            0xcb => {
                if env.is_alt_index() {
                    env.load_displacement();
                    &self.prefix_cb_indexed[env.advance_pc() as usize]
                } else {
                    &self.prefix_cb[env.advance_pc() as usize]
                }
            },
            0xed => {
                env.clear_index(); // With ed, the current prefix is ignored
                &self.prefix_ed[env.advance_pc() as usize]
            },
            _ => {
                if self.has_displacement[code as usize] && env.is_alt_index() {
                    env.load_displacement();
                }
                &self.no_prefix[code as usize]
            }
        }
    }
}

impl DecoderZ80 {
    pub fn new() -> DecoderZ80 {
        DecoderZ80 {
            no_prefix: no_prefix_opcodes(),
            prefix_cb: cb_prefix_opcodes(),
            prefix_cb_indexed: cb_indexed_prefix_opcodes(),
            prefix_ed: ed_prefix_opcodes(),
            has_displacement: displacements(),
        }
    }
}

fn no_prefix_opcodes() -> [Opcode;256] {
    let mut opcodes_vector = Vec::with_capacity(256);
    for c in 0..=255 {
        let p = DecodingHelper::parts(c);
        let mut opcode = match p.x {
            0 => match p.z {
                0 => match p.y { // Relative jumps and assorted ops.
                    0 => build_nop(), // NOP
                    1 => build_ex_af(), // EX AF, AF'
                    2 => build_djnz(), // DJNZ d
                    3 => build_jr_unconditional(), // JR d
                    _ /*4..=7*/ => build_jr_eq(CC[p.y-4]),
                },
                1 => match p.q {
                    0 => build_ld_rr_nn(RP[p.p]), // LD rr, nn -- 16-bit load add
                    _ /*1*/ => build_add_hl_rr(RP[p.p]), // ADD HL, rr -- 16-bit add
                },
                2 => match p.q {
                    0 =>  match p.p {
                        0 => build_ld_prr_a(Reg16::BC), // LD (BC), A
                        1 => build_ld_prr_a(Reg16::DE), // LD (DE), A
                        2 => build_ld_pnn_rr(Reg16::HL, true), // LD (nn), HL
                        _ /*3*/ => build_ld_pnn_a(), // LD (nn), A
                    },
                    _ /*1*/ =>  match p.p {
                        0 => build_ld_a_prr(Reg16::BC), // LD A, (BC)
                        1 => build_ld_a_prr(Reg16::DE), // LD A, (DE)
                        2 => build_ld_rr_pnn(Reg16::HL, true), // LD HL, (nn)
                        _ /*3*/ => build_ld_a_pnn(), // LD A, (nn)
                    }
                },
                3 => match p.q {
                    0 =>  build_inc_dec_rr(RP[p.p], true), // INC rr -- 16-bit inc
                    _ /*1*/ =>  build_inc_dec_rr(RP[p.p], false), // DEC rr -- 16-bit dec                      
                },
                4 => build_inc_r(R[p.y]), // INC r -- 8 bit inc
                5 => build_dec_r(R[p.y]), // DEC r -- 8 bit dec
                6 => build_ld_r_n(R[p.y]), // LD r, n -- 8 bit load imm
                _ /*7*/ => match p.y {
                    0..=3 => build_rot_r(Reg8::A, ROT[p.y], true, false), // rotA
                    4 => build_daa(), // DAA, decimal adjust A
                    5 => build_cpl(), // CPL, complement adjust A
                    6 => build_scf(), // SCF, set carry flag
                    _ /*7*/ => build_ccf(), // CCF, clear carry flag
                },
            },
            1 => match (p.z, p.y) {
                (6, 6) => build_halt(), // HALT, exception instead of LD (HL), (HL)
                _ => build_ld_r_r(R[p.y], R[p.z], false), // LD r[y], r[z] -- 8 bit load imm
            },
            2 => build_operator_a_r(R[p.z], ALU[p.y]), // alu A, r
            _ /*3*/ => match p.z {
                0 => build_ret_eq(CC[p.y]), // RET cc
                1 => match p.q {
                    0 => build_pop_rr(RP2[p.p]), // POP rr
                    _ /*1*/ => match p.p {
                        0 => build_ret(), // RET
                        1 => build_exx(), // EXX
                        2 => build_jp_hl(), // JP HL
                        _ /*3*/ => build_ld_sp_hl(), // LD SP, HL
                    },
                },
                2 => build_jp_eq(CC[p.y]), // JP cc, nn
                3 => match p.y {
                    0 => build_jp_unconditional(), // JP nn
                    1 => build_not_an_opcode(), // CB prefix
                    2 => build_out_n_a(),  // OUT (n), A
                    3 => build_in_a_n(),   // IN A, (n)
                    4 => build_ex_psp_hl(), // EX (SP), HL
                    5 => build_ex_de_hl(),  // EX DE, HL
                    6 => build_disable_interrupts(), // DI
                    _ /*7*/ => build_enable_interrupts(),  // EI
                }
                4 => build_call_eq(CC[p.y]),
                5 => match p.q {
                    0 => build_push_rr(RP2[p.p]), // PUSH rr
                    _ /*1*/ => match p.p {
                        0 => build_call(), // Call nn
                        1 => build_not_an_opcode(), // DD prefix
                        2 => build_not_an_opcode(), // ED prefix
                        _ /*3*/ => build_not_an_opcode(), // FD prefix
                    },
                },
                6 => build_operator_a_n(ALU[p.y]), // alu A, n
                _ /*7*/ => build_rst(p.y as u8 * 8), // RST
            },
        };
        opcode.cycles = NO_PREFIX_CYCLES[c as usize];
        opcode.cycles_conditional = opcode.cycles;
        opcodes_vector.push(opcode);
    }

    let mut opcodes = opcodes_vector.try_into().unwrap_or_else(|_| { panic!("missing opcodes")});
    load_cycle_information_no_prefix(&mut opcodes);
    opcodes
}

fn cb_prefix_opcodes() -> [Opcode;256] {
    let mut opcodes_vector = Vec::with_capacity(256);

    for c in 0..=255 {
        let p = DecodingHelper::parts(c);
        let mut opcode = match p.x {
            0 => build_rot_r(R[p.z], ROT[p.y], false, false), // Shifts
            1 => build_bit_r(p.y as u8, R[p.z]), // BIT
            2 => build_set_res_r(p.y as u8, R[p.z], false), // RES
            _ /*3*/ => build_set_res_r(p.y as u8, R[p.z], true), // SET
        };
        opcode.cycles = PREFIX_CB_CYCLES[c as usize];
        opcode.cycles_conditional = opcode.cycles;
        opcodes_vector.push(opcode);
    }

    opcodes_vector.try_into().unwrap_or_else(|_| { panic!("missing opcodes")})
}

fn cb_indexed_prefix_opcodes() -> [Opcode;256] {
    let mut opcodes_vector = Vec::with_capacity(256);

    for c in 0..=255 {
        let p = DecodingHelper::parts(c);
        let mut opcode = match p.x {
            0 => build_rot_r(R[p.z], ROT[p.y], false, true), // Shifts
            1 => build_bit_r(p.y as u8, R[p.z]), // BIT
            2 => build_indexed_set_res_r(p.y as u8, R[p.z], false), // RES
            _ /*3*/ => build_indexed_set_res_r(p.y as u8, R[p.z], true), // SET
        };
        // 23 cycles except for BIT that is 20
        opcode.cycles = if (c & 0xc0) == 0x40 {20} else {23};
        opcode.cycles_conditional = opcode.cycles;
        opcodes_vector.push(opcode);
    }

    opcodes_vector.try_into().unwrap_or_else(|_| { panic!("missing opcodes")})
}


fn ed_prefix_opcodes() -> [Opcode;256] {
    let mut opcodes_vector = Vec::with_capacity(256);

    for c in 0..=255 {
        let p = DecodingHelper::parts(c);
        let mut opcode = match p.x {
            0 | 3 => build_noni_nop(), // Invalid instruction NONI + NOP
            1 => match p.z {
                0 => match p.y {
                    6 => build_in_0_c(), // IN (C)
                    _ => build_in_r_c(R[p.y]), // IN r, (C)
                }
                1 => match p.y {
                    6 => build_out_c_0(), // OUT (C), 0
                    _ => build_out_c_r(R[p.y]), // OUT (C), r
                }
                2 => match p.q {
                    0 => build_sbc_hl_rr(RP[p.p]), // SBC HL, rr
                    _ /*1*/ => build_adc_hl_rr(RP[p.p]), // ADC HL, rr
                },
                3 => match p.q {
                    0 => build_ld_pnn_rr(RP[p.p], false), // LD (nn), rr -- 16 bit loading
                    _ /*1*/ => build_ld_rr_pnn(RP[p.p], false), // LD rr, (nn) -- 16 bit loading
                },
                4 => build_neg(), // NEG
                5 => match p.y {
                    1 => build_reti(), // RETI
                    _ => build_retn()  // RETN
                }
                6 => build_im(IM[p.y]), // IM #
                _ /*7*/ => match p.y {
                    0 => build_ld_r_r(Reg8::I, Reg8::A, true), // LD I, A
                    1 => build_ld_r_r(Reg8::R, Reg8::A, true), // LD R, A
                    2 => build_ld_r_r(Reg8::A, Reg8::I, true), // LD A, I
                    3 => build_ld_r_r(Reg8::A, Reg8::R, true), // LD A, R
                    4 => build_rxd(ShiftDir::Right, "RRD"), // RRD
                    5 => build_rxd(ShiftDir::Left, "RLD"),  // RLD
                    6 => build_nop(), // NOP
                    _ /*7*/ => build_nop(), // NOP
                },
            },
            _ /*2*/ => if p.z <= 3 && p.y >= 4 {
                // Table "bli"
                match p.z {
                    0 => build_ld_block( BLI_A[p.y-4]), // Block LDxx
                    1 => build_cp_block( BLI_A[p.y-4]), // Block CPxx
                    2 => build_in_block( BLI_A[p.y-4]), // Block INxx
                    _ /*3*/ => build_out_block(BLI_A[p.y-4]), // Block OUTxx
                 }
            } else {
                 build_noni_nop() // NONI + NOP
            },
        };
        opcode.cycles = PREFIX_ED_CYCLES[c as usize];
        opcode.cycles_conditional = opcode.cycles;
        opcodes_vector.push(opcode);
    }

    let mut opcodes = opcodes_vector.try_into().unwrap_or_else(|_| { panic!("missing opcodes")});
    load_cycle_information_prefix_ed(&mut opcodes);
    opcodes
}

fn displacements() -> [bool; 256] {
    let mut disps = [false; 256];
    disps[0x34] = true;
    disps[0x35] = true;
    disps[0x36] = true;
    disps[0x46] = true;
    disps[0x4e] = true;
    disps[0x56] = true;
    disps[0x5e] = true;
    disps[0x66] = true;
    disps[0x6e] = true;
    disps[0x70] = true;
    disps[0x71] = true;
    disps[0x72] = true;
    disps[0x73] = true;
    disps[0x74] = true;
    disps[0x75] = true;
    disps[0x77] = true;
    disps[0x7e] = true;
    disps[0x86] = true;
    disps[0x8e] = true;
    disps[0x96] = true;
    disps[0x9e] = true;
    disps[0xa6] = true;
    disps[0xae] = true;
    disps[0xb6] = true;
    disps[0xbe] = true;
    disps
}

fn load_cycle_information_no_prefix(opcodes: &mut [Opcode; 256]) {
    opcodes[0x10].cycles_conditional =  8;
    opcodes[0x20].cycles_conditional =  8;
    opcodes[0x28].cycles_conditional =  8;
    opcodes[0x30].cycles_conditional =  8;
    opcodes[0x38].cycles_conditional =  8;

    opcodes[0xc0].cycles_conditional =  5;
    opcodes[0xc2].cycles_conditional =  7;
    opcodes[0xc4].cycles_conditional = 10;
    opcodes[0xc8].cycles_conditional =  5;
    opcodes[0xca].cycles_conditional =  7;
    opcodes[0xcc].cycles_conditional = 10;

    opcodes[0xd0].cycles_conditional =  5;
    opcodes[0xd2].cycles_conditional =  7;
    opcodes[0xd4].cycles_conditional = 10;
    opcodes[0xd8].cycles_conditional =  5;
    opcodes[0xda].cycles_conditional =  7;
    opcodes[0xdc].cycles_conditional = 10;

    opcodes[0xe0].cycles_conditional =  5;
    opcodes[0xe2].cycles_conditional =  7;
    opcodes[0xe4].cycles_conditional = 10;
    opcodes[0xe8].cycles_conditional =  5;
    opcodes[0xea].cycles_conditional =  7;
    opcodes[0xec].cycles_conditional = 10;

    opcodes[0xf0].cycles_conditional =  5;
    opcodes[0xf2].cycles_conditional =  7;
    opcodes[0xf4].cycles_conditional = 10;
    opcodes[0xf8].cycles_conditional =  5;
    opcodes[0xfa].cycles_conditional =  7;
    opcodes[0xfc].cycles_conditional = 10;
}

fn load_cycle_information_prefix_ed(opcodes: &mut [Opcode; 256]) {
    opcodes[0xb0].cycles_conditional = 16;
    opcodes[0xb1].cycles_conditional = 16;
    opcodes[0xb2].cycles_conditional = 16;
    opcodes[0xb3].cycles_conditional = 16;
    opcodes[0xb8].cycles_conditional = 16;
    opcodes[0xb9].cycles_conditional = 16;
    opcodes[0xba].cycles_conditional = 16;
    opcodes[0xbb].cycles_conditional = 16;
}

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
const IM: [u8; 8] = [0, 0, 1, 2, 0, 0, 1, 2];

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

const BLI_A: [(bool, bool, &str); 4] = [
    (true,  false, "I"),
    (false, false, "D"),
    (true,  true, "IR"),
    (false, true, "DR")
];


// From https://spectrumforeveryone.com/technical/z80-processor-instructions/
const NO_PREFIX_CYCLES: [u8; 256] = [
     4, 10,  7,  6,  4,  4,  7,  4,  4, 11,  7,  6,  4,  4,  7,  4,
    13, 10,  7,  6,  4,  4,  7,  4, 12, 11,  7,  6,  4,  4,  7,  4,
    12, 10, 16,  6,  4,  4,  7,  4, 12, 11, 16,  6,  4,  4,  7,  4,
    12, 10, 13,  6, 11, 11, 10,  4, 12, 11, 13,  6,  4,  4,  7,  4,
     4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,
     4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,
     4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,
     7,  7,  7,  7,  7,  7,  4,  7,  4,  4,  4,  4,  4,  4,  7,  4,
     4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,
     4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,
     4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,
     4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,
    11, 10, 12, 10, 17, 11,  7, 11, 11, 10, 12,  0, 17, 17,  7, 11,
    11, 10, 12, 11, 17, 11,  7, 11, 11,  4, 12, 11, 17 , 0,  7, 11,
    11, 10, 12, 19, 17, 11,  7, 11, 11,  4, 12,  4, 17,  0,  7, 11,
    11, 10, 12,  4, 17, 11,  7, 11, 11,  6, 12,  4, 17,  0,  7, 11,
];

const PREFIX_CB_CYCLES: [u8; 256] = [
     8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8,
     8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8,
     8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8,
     8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8,
     8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8, 12,  8,
     8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8, 12,  8,
     8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8, 12,  8,
     8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8, 12,  8,
     8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8,
     8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8,
     8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8,
     8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8,
     8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8,
     8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8,
     8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8,
     8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8,
];

const PREFIX_ED_CYCLES: [u8; 256] = [
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    12, 12, 15, 20,  8, 14,  8,  9, 12, 12, 15, 20,  8, 14,  8,  9,
    12, 12, 15, 20,  8, 14,  8,  9, 12, 12, 15, 20,  8, 14,  8,  9,
    12, 12, 15, 20,  8, 14,  8, 18, 12, 12, 15, 20,  8, 14,  8, 18,
    12, 12, 15, 20,  8, 14,  8,  0, 12, 12, 15, 20,  8, 14,  8,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    16, 16, 16, 16,  0,  0,  0,  0, 16, 16, 16, 16,  0,  0,  0,  0,
    21, 21, 21, 21,  0,  0,  0,  0, 21, 21, 21, 21,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
];

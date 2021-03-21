#[rustfmt::skip]
pub static MNEMONICS: [&str; 256] = [
    "brk",   "inc^",  "",      "",      "drp1",  "swp1",  "rof1",  "fcm1",  "inc1",  "add1",  "adc1",  "bnt1",  "ior1",  "shl1",  "eqz1",  "gth1",
    "nop",   "add^",  "drp^",  "",      "drp2",  "swp2",  "rof2",  "fcm2",  "inc2",  "add2",  "adc2",  "bnt2",  "ior2",  "shl2",  "eqz2",  "gth2",
    "jmp",   "dec^",  "dup^",  "",      "drp3",  "swp3",  "rof3",  "fcm3",  "inc3",  "add3",  "adc3",  "bnt3",  "ior3",  "shl3",  "eqz3",  "gth3",
    "jsr",   "sub^",  "swp^",  "",      "drp4",  "swp4",  "rof4",  "fcm4",  "inc4",  "add4",  "adc4",  "bnt4",  "ior4",  "shl4",  "eqz4",  "gth4",
    "nip",   "eqz^",  "ovr^",  "sts1",  "dup1",  "ovr1",  "rob1",  "stm1",  "dec1",  "sub1",  "sbb1",  "and1",  "xor1",  "shr1",  "nez1",  "lth1",
    "dsp",   "nez^",  "rof^",  "sts2",  "dup2",  "ovr2",  "rob2",  "stm2",  "dec2",  "sub2",  "sbb2",  "and2",  "xor2",  "shr2",  "nez2",  "lth2",
    "sei",   "gth^",  "rob^",  "sts3",  "dup3",  "ovr3",  "rob3",  "stm3",  "dec3",  "sub3",  "sbb3",  "and3",  "xor3",  "shr3",  "nez3",  "lth3",
    "cli",   "lth^",  "ext",   "sts4",  "dup4",  "ovr4",  "rob4",  "stm4",  "dec4",  "sub4",  "sbb4",  "and4",  "xor4",  "shr4",  "nez4",  "lth4",
    "brk?",  "inc^?", "lit^?", "lit1?", "drp1?", "swp1?", "rof1?", "fcm1?", "inc1?", "add1?", "adc1?", "bnt1?", "ior1?", "shl1?", "eqz1?", "gth1?",
    "nop?",  "add^?", "drp^?", "lit2?", "drp2?", "swp2?", "rof2?", "fcm2?", "inc2?", "add2?", "adc2?", "bnt2?", "ior2?", "shl2?", "eqz2?", "gth2?",
    "jmp?",  "dec^?", "dup^?", "lit3?", "drp3?", "swp3?", "rof3?", "fcm3?", "inc3?", "add3?", "adc3?", "bnt3?", "ior3?", "shl3?", "eqz3?", "gth3?",
    "jsr?",  "sub^?", "swp^?", "lit4?", "drp4?", "swp4?", "rof4?", "fcm4?", "inc4?", "add4?", "adc4?", "bnt4?", "ior4?", "shl4?", "eqz4?", "gth4?",
    "nip?",  "eqz^?", "ovr^?", "sts1?", "dup1?", "ovr1?", "rob1?", "stm1?", "dec1?", "sub1?", "sbb1?", "and1?", "xor1?", "shr1?", "nez1?", "lth1?",
    "dsp?",  "nez^?", "rof^?", "sts2?", "dup2?", "ovr2?", "rob2?", "stm2?", "dec2?", "sub2?", "sbb2?", "and2?", "xor2?", "shr2?", "nez2?", "lth2?",
    "sei?",  "gth^?", "rob^?", "sts3?", "dup3?", "ovr3?", "rob3?", "stm3?", "dec3?", "sub3?", "sbb3?", "and3?", "xor3?", "shr3?", "nez3?", "lth3?",
    "cli?",  "lth^?", "ext?",  "sts4?", "dup4?", "ovr4?", "rob4?", "stm4?", "dec4?", "sub4?", "sbb4?", "and4?", "xor4?", "shr4?", "nez4?", "lth4?",
];

#[derive(Debug, Clone, Copy)]
pub enum YotType {
    Y8 = 1,
    Y16 = 2,
    Y32 = 4,
    Y64 = 8,
}

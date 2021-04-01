#[rustfmt::skip]
pub static MNEMONICS: [&str; 256] = [
"brk", "", "inc^", "", "fcm1", "drp1", "swp1", "rtf1", "inc1", "eqz1", "add1", "adc1", "bnt1", "ior1", "shl1", "rtl1",
"nop", "", "dec^", "", "fcm2", "drp2", "swp2", "rtf2", "inc2", "eqz2", "add2", "adc2", "bnt2", "ior2", "shl2", "rtl2",
"jmp", "drp^", "eqz^", "", "fcm4", "drp4", "swp4", "rtf4", "inc4", "eqz4", "add4", "adc4", "bnt4", "ior4", "shl4", "rtl4",
"jsr", "dup^", "nez^", "", "fcm8", "drp8", "swp8", "rtf8", "inc8", "eqz8", "add8", "adc8", "bnt8", "ior8", "shl8", "rtl8",
"nip", "swp^", "add^", "sts1", "stm1", "dup1", "ovr1", "rtb1", "dec1", "nez1", "sub1", "sbb1", "and1", "xor1", "shr1", "rtr1",
"dsp", "ovr^", "sub^", "sts2", "stm2", "dup2", "ovr2", "rtb2", "dec2", "nez2", "sub2", "sbb2", "and2", "xor2", "shr2", "rtr2",
"sei", "rtf^", "adc^", "sts4", "stm4", "dup4", "ovr4", "rtb4", "dec4", "nez4", "sub4", "sbb4", "and4", "xor4", "shr4", "rtr4",
"cli", "rtb^", "sbb^", "sts8", "stm8", "dup8", "ovr8", "rtb8", "dec8", "nez8", "sub8", "sbb8", "and8", "xor8", "shr8", "rtr8",
"brk?", "", "inc^?", "", "fcm1?", "drp1?", "swp1?", "rtf1?", "inc1?", "eqz1?", "add1?", "adc1?", "bnt1?", "ior1?", "shl1?", "rtl1?",
"nop?", "", "dec^?", "", "fcm2?", "drp2?", "swp2?", "rtf2?", "inc2?", "eqz2?", "add2?", "adc2?", "bnt2?", "ior2?", "shl2?", "rtl2?",
"jmp?", "drp^?", "eqz^?", "", "fcm4?", "drp4?", "swp4?", "rtf4?", "inc4?", "eqz4?", "add4?", "adc4?", "bnt4?", "ior4?", "shl4?", "rtl4?",
"jsr?", "dup^?", "nez^?", "", "fcm8?", "drp8?", "swp8?", "rtf8?", "inc8?", "eqz8?", "add8?", "adc8?", "bnt8?", "ior8?", "shl8?", "rtl8?",
"nip?", "swp^?", "add^?", "sts1?", "stm1?", "dup1?", "ovr1?", "rtb1?", "dec1?", "nez1?", "sub1?", "sbb1?", "and1?", "xor1?", "shr1?", "rtr1?",
"dsp?", "ovr^?", "sub^?", "sts2?", "stm2?", "dup2?", "ovr2?", "rtb2?", "dec2?", "nez2?", "sub2?", "sbb2?", "and2?", "xor2?", "shr2?", "rtr2?",
"sei?", "rtf^?", "adc^?", "sts4?", "stm4?", "dup4?", "ovr4?", "rtb4?", "dec4?", "nez4?", "sub4?", "sbb4?", "and4?", "xor4?", "shr4?", "rtr4?",
"cli?", "rtb^?", "sbb^?", "sts8?", "stm8?", "dup8?", "ovr8?", "rtb8?", "dec8?", "nez8?", "sub8?", "sbb8?", "and8?", "xor8?", "shr8?", "rtr8?",
];

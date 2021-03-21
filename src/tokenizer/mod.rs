use super::span::{Location, Span, Spanned, Spanning};
use error::Error;
use token::{Token, DataLiteral};
use unicode_segmentation::UnicodeSegmentation;

pub mod error;
pub mod token;

const PRIMITIVE_INSTRUCTIONS: &'static [&'static str] = &[
    "brk", "nop", "jmp", "jsr", "nip", "dsp", "sei", "cli", "sts1", "fcm1", "stm1", "adc1", "sbb1",
    "bnt1", "and1", "ior1", "xor1", "shl1", "shr1", "sts2", "fcm2", "stm2", "adc2", "sbb2", "bnt2",
    "and2", "ior2", "xor2", "shl2", "shr2", "sts4", "fcm4", "stm4", "adc4", "sbb4", "bnt4", "and4",
    "ior4", "xor4", "shl4", "shr4", "sts8", "fcm8", "stm8", "adc8", "sbb8", "bnt8", "and8", "ior8",
    "xor8", "shl8", "shr8", "inc1", "add1", "dec1", "sub1", "eqz1", "nez1", "gth1", "lth1", "drp1",
    "dup1", "swp1", "ovr1", "rof1", "rob1", "inc2", "add2", "dec2", "sub2", "eqz2", "nez2", "gth2",
    "lth2", "drp2", "dup2", "swp2", "ovr2", "rof2", "rob2", "inc4", "add4", "dec4", "sub4", "eqz4",
    "nez4", "gth4", "lth4", "drp4", "dup4", "swp4", "ovr4", "rof4", "rob4", "inc8", "add8", "dec8",
    "sub8", "eqz8", "nez8", "gth8", "lth8", "drp8", "dup8", "swp8", "ovr8", "rof8", "rob8", "inc^",
    "add^", "dec^", "sub^", "eqz^", "nez^", "gth^", "lth^", "drp^", "dup^", "swp^", "ovr^", "rof^",
    "rob^",
];

fn is_digit(string: &str) -> bool {
    matches!(string.chars().next(), Some(ch) if ch.is_digit(16))
}

fn to_digit(string: &str) -> Option<u32> {
    string.chars().next().and_then(|ch| ch.to_digit(16))
}

fn is_delimiter(string: &str) -> bool {
    is_whitespace(string)
}

fn is_whitespace(string: &str) -> bool {
    match string {
        "\n" | "\r" | "\r\n" => return true,
        _ => {}
    }
    matches!(string.chars().next(), Some(ch) if ch.is_whitespace())
}

fn scan<'a>(string: &'a str) -> Vec<Spanned<&'a str>> {
    string
        .graphemes(true)
        .scan(Location::new(), |location, grapheme| {
            let current_location = *location;
            (*location).offset += grapheme.chars().count();
            Some(Spanned {
                node: grapheme,
                span: Span {
                    from: current_location,
                    to: *location,
                },
            })
        })
        .collect()
}

fn parse_data_literal<'a>(symbols: &[Spanned<&'a str>], i: &mut usize) -> Result<Spanned<DataLiteral>, Error> {
    let os = symbols.get(*i).unwrap().span;
    let mut oe = os;
    let mut value: u64 = 0;
    let mut length: usize = 0;
    *i += 1;
    loop {
        match symbols.get(*i) {
            Some(Spanned { node: c, span: o }) if is_digit(c) => {
                value = (value << 4) + to_digit(c).unwrap() as u64;
                length += 1;
                *i += 1;
            }
            Some(Spanned { node: c, span: o }) if !is_delimiter(c) => {
                return Err(Error::DigitInvalid(c.chars().next().unwrap(), *o));
            }
            _ => {
                return Ok(match length {
                    0 => return Err(Error::DigitExpected(os)),
                    1..=2 => DataLiteral::U8(value as u8),
                    3..=4 => DataLiteral::U16(value as u16),
                    5..=8 => DataLiteral::U32(value as u32),
                    9..=16 => DataLiteral::U64(value as u64),
                    _ => return Err(Error::DataLiteralTooLarge(Span::combine(&os, &oe))),
                }.spanning(Span::combine(&os, &oe)));
            }
        }
    }
}

fn parse_address_literal<'a>(symbols: &[Spanned<&'a str>], i: &mut usize) -> Result<Spanned<u64>, Error> {
    let os = symbols.get(*i).unwrap().span;
    let mut oe = os;
    let mut value: u64 = 0;
    let mut length: usize = 0;
    *i += 1;
    loop {
        match symbols.get(*i) {
            Some(Spanned { node: c, span: o }) if is_digit(c) => {
                value = (value << 4) + to_digit(c).unwrap() as u64;
                length += 1;
                *i += 1;
            }
            Some(Spanned { node: c, span: o }) if !is_delimiter(c) => {
                return Err(Error::DigitInvalid(c.chars().next().unwrap(), *o));
            }
            _ => {
                return Ok(match length {
                    0 => return Err(Error::DigitExpected(os)),
                    _ => value,
                }.spanning(Span::combine(&os, &oe)));
            }
        }
    }
}

pub(super) fn tokenize(input_string: &str) -> Result<Vec<Spanned<Token>>, Vec<Error>> {
    let symbols = scan(input_string);

    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut i = 0;

    'tokens: loop {
        'gap: loop {
            match symbols.get(i).as_ref() {
                Some(Spanned { node: c, .. }) if is_whitespace(c) => {
                    i += 1;
                }
                Some(Spanned { node: ";", .. }) => {
                    i += 1;
                    let mut comment = true;
                    // comment
                    while comment {
                        match symbols.get(i) {
                            Some(Spanned { node: "\n", .. }) => {
                                i += 1;
                                comment = false;
                            }
                            Some(_) => {
                                i += 1;
                            }
                            None => break 'gap,
                        }
                    }
                }
                _ => break 'gap,
            };
        };
        match symbols.get(i) {
            Some(Spanned { node: "#", .. }) => {
                match parse_data_literal(&symbols, &mut i) {
                    Ok(Spanned { node: dl, span }) => {
                        tokens.push(Token::DataLiteral(dl).spanning(span));
                    }
                    Err(err) => {
                        errors.push(err);
                    }
                };
            }
            Some(Spanned { node: "%", .. }) => {
                match parse_address_literal(&symbols, &mut i) {
                    Ok(Spanned { node: value, span }) => {
                        tokens.push(Token::AddressLiteral(value).spanning(span));
                    }
                    Err(err) => {
                        errors.push(err);
                    }
                }
            }
            Some(Spanned { node: c, span }) => {
                // errors.push(Error::SymbolInvalid((*c).to_string(), *span));
                i += 1;
            }
            None => break 'tokens,
        }
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}

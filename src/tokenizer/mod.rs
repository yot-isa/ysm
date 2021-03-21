use super::instruction::MNEMONICS;
use super::span::{Location, Span, Spanned, Spanning};
use error::Error;
use token::{DataLiteral, Token};
use unicode_segmentation::UnicodeSegmentation;

pub mod error;
pub mod token;

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

fn parse_identifier<'a>(
    symbols: &[Spanned<&'a str>],
    i: &mut usize,
    skip_first: bool,
) -> Result<Spanned<String>, Error> {
    let os = symbols.get(*i).unwrap().span;
    let mut oe = os;
    let mut label: String = String::new();
    let mut length: usize = 0;
    if skip_first {
        *i += 1;
    }
    loop {
        match symbols.get(*i) {
            Some(Spanned { node: c, span: o }) if !is_delimiter(c) => {
                label.push_str(c);
                oe = *o;
                length += 1;
                *i += 1;
            }
            _ => {
                return Ok(match length {
                    0 => return Err(Error::IdentifierExpected(os)),
                    _ => label,
                }
                .spanning(Span::combine(&os, &oe)));
            }
        }
    }
}

fn parse_data_literal<'a>(
    symbols: &[Spanned<&'a str>],
    i: &mut usize,
) -> Result<Spanned<DataLiteral>, Error> {
    let os = symbols.get(*i).unwrap().span;
    let mut oe = os;
    let mut value: u64 = 0;
    let mut length: usize = 0;
    *i += 1;
    loop {
        match symbols.get(*i) {
            Some(Spanned { node: c, span: o }) if is_digit(c) => {
                value = (value << 4) + to_digit(c).unwrap() as u64;
                oe = *o;
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
                }
                .spanning(Span::combine(&os, &oe)));
            }
        }
    }
}

fn parse_address_literal<'a>(
    symbols: &[Spanned<&'a str>],
    i: &mut usize,
) -> Result<Spanned<u64>, Error> {
    let os = symbols.get(*i).unwrap().span;
    let mut oe = os;
    let mut value: u64 = 0;
    let mut length: usize = 0;
    *i += 1;
    loop {
        match symbols.get(*i) {
            Some(Spanned { node: c, span: o }) if is_digit(c) => {
                value = (value << 4) + to_digit(c).unwrap() as u64;
                oe = *o;
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
                }
                .spanning(Span::combine(&os, &oe)));
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
        }
        match symbols.get(i) {
            Some(Spanned { node: ":", .. }) => match parse_identifier(&symbols, &mut i, true) {
                Ok(Spanned { node: ld, span }) => {
                    tokens.push(Token::LabelDefinition(ld).spanning(span))
                }
                Err(err) => errors.push(err),
            },
            Some(Spanned { node: "@", .. }) => match parse_identifier(&symbols, &mut i, true) {
                Ok(Spanned { node: ld, span }) => {
                    tokens.push(Token::LabelLiteral(ld).spanning(span))
                }
                Err(err) => errors.push(err),
            },
            Some(Spanned { node: "#", .. }) => {
                match parse_data_literal(&symbols, &mut i) {
                    Ok(Spanned { node: dl, span }) => {
                        tokens.push(Token::DataLiteral(dl).spanning(span))
                    }
                    Err(err) => errors.push(err),
                };
            }
            Some(Spanned { node: "%", .. }) => match parse_address_literal(&symbols, &mut i) {
                Ok(Spanned { node: value, span }) => {
                    tokens.push(Token::AddressLiteral(value).spanning(span))
                }
                Err(err) => errors.push(err),
            },
            Some(Spanned { node: c, .. }) => match parse_identifier(&symbols, &mut i, false) {
                Ok(Spanned { node: id, span }) => {
                    if let Some(opcode) = MNEMONICS.iter().position(|&m| m == id) {
                        tokens.push(
                            Token::PrimitiveInstruction(opcode as u8).spanning(span),
                        );
                    } else {
                        tokens.push(Token::SubroutineJump(id).spanning(span));
                    }
                }
                Err(_) => unreachable!(),
            },
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

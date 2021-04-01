use super::span::{Location, Span, Spanned, Spanning};
pub use error::Error;
use token::Token;
use unicode_segmentation::UnicodeSegmentation;
use super::{InstructionKind, get_instruction_kind};

pub mod error;
pub mod token;

fn is_digit(string: &str) -> bool {
    matches!(string.chars().next(), Some(ch) if ch.is_digit(16))
}

fn to_digit(string: &str) -> Option<u8> {
    string.chars().next().and_then(|ch| ch.to_digit(16).map(|v| v as u8))
}

fn is_delimiter(string: &str) -> bool {
    is_whitespace(string) || string == "{" || string == "}"
}

fn is_whitespace(string: &str) -> bool {
    match string {
        "\n" | "\r" | "\r\n" => return true,
        _ => {}
    }
    matches!(string.chars().next(), Some(ch) if ch.is_whitespace())
}

fn scan(string: &str, file_id: usize) -> Vec<Spanned<&str>> {
    string
        .graphemes(true)
        .scan(Location { offset: 0 }, |location, grapheme| {
            let current_location = *location;
            (*location).offset += grapheme.chars().count();
            Some(Spanned {
                node: grapheme,
                span: Span {
                    file_id,
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
                    0 => return Err(Error::IdentifierExpected { span: os }),
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
) -> Result<Spanned<Vec<u8>>, Error> {
    let os = symbols.get(*i).unwrap().span;
    let mut oe = os;
    let mut byte: u8 = 0;
    let mut even_nibble: bool = false;
    let mut byte_vector: Vec<u8> = Vec::new();
    *i += 1;
    loop {
        match symbols.get(*i) {
            Some(Spanned { node: c, span: o }) if is_digit(c) => {
                byte = (byte << 4) + to_digit(c).unwrap();
                if even_nibble {
                    byte_vector.push(byte);
                    byte = 0;
                }
                oe = *o;
                even_nibble = !even_nibble;
                *i += 1;
            }
            Some(Spanned { node: c, span: o }) if !is_delimiter(c) => {
                return Err(Error::DigitInvalid { digit: c.chars().next().unwrap(), span: *o });
            }
            _ => {
                if byte_vector.is_empty() {
                    return Err(Error::DigitExpected { span: os });
                }
                return Ok(byte_vector.spanning(Span::combine(&os, &oe)));
            }
        }
    }
}

pub(super) fn tokenize(input_string: &str, file_id: usize) -> Result<Vec<Spanned<Token>>, Vec<Error>> {
    let symbols = scan(input_string, file_id);

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
            Some(Spanned { node: "{", span }) => {
                tokens.push(Token::OpeningBrace.spanning(*span));
                i += 1;
            },
            Some(Spanned { node: "}", span }) => {
                tokens.push(Token::ClosingBrace.spanning(*span));
                i += 1;
            },
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
            Some(Spanned { .. }) => match parse_identifier(&symbols, &mut i, false) {
                Ok(Spanned { node: id, span }) => {
                    if let Some(instruction_kind) = get_instruction_kind(&id) {
                        tokens.push(Token::PrimitiveInstruction(instruction_kind).spanning(span));
                    } else {
                        tokens.push(Token::SubroutineJump(id).spanning(span));
                    }
                }
                Err(_) => unreachable!(),
            },
            None => break 'tokens,
        }
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}

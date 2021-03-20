use super::span::{Location, Span, Spanned, Spanning};
use error::Error;
use token::Token;
use unicode_segmentation::UnicodeSegmentation;

pub mod error;
pub mod token;

fn scan<'a>(string: &'a str) -> Vec<Spanned<&'a str>> {
    string
        .graphemes(true)
        .scan(Location::new(), |location, grapheme| {
            let current_location = *location;
            (*location).column += grapheme.chars().count();
            if grapheme == "\n" || grapheme == "\r" || grapheme == "\r\n" {
                (*location).line += 1;
                (*location).column = 0;
            }
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

pub fn tokenize(input_string: &str) -> Result<Vec<Spanned<Token>>, Vec<Spanned<Error>>> {
    let symbols = scan(input_string);

    println!("{:#?}", symbols);

    Ok(vec![])
}

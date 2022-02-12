use std::process::exit;
use clap::lazy_static::lazy_static;
use EscapeResult::Inescapable;
use crate::args::SayArgs;
use crate::EscapeResult::{Char, Return};

mod args;
mod test;

lazy_static! { static ref ARGS: SayArgs = args::args();}

fn main() {
    let mut text = ARGS.text.join(" ");
    text = if ARGS.backslash_interpretation { interpretation(text) } else { text };
    print!("{}{}", text, if ARGS.trim_new_line { "" } else { "\n" })
}

fn interpretation(text: String) -> String {
    let mut result = String::with_capacity(text.len());
    let mut is_escaped = false;
    for char in text.chars() {
        if is_escaped {
            match escaped(char) {
                Char(c) => {
                    is_escaped = false;
                    result.push(c);
                }
                Return => {
                    print!("{}", result);
                    exit(0)
                }
                Inescapable => {
                    is_escaped = false;
                    result = result + format!("\\{}", char).as_str()
                }
            }
        } else {
            match char {
                '\\' => { is_escaped = true }
                _ => { result.push(char) }
            }
        }
    }
    return result;
}

fn escaped(c: char) -> EscapeResult {
    Char(match c {
        '\\' => '\\',
        'a' => '\x07',
        'b' => '\x08',
        'e' => '\x1B',
        'f' => '\x0C',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        'v' => '\x0B',
        'c' => { return Return; }
        _ => { return Inescapable; }
    })
}

enum EscapeResult {
    Char(char),
    Return,
    Inescapable,
}
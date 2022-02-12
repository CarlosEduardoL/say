use std::process::exit;
use clap::lazy_static::lazy_static;
use crate::args::SayArgs;

mod args;

static EMPTY: String = String::new();
lazy_static! { static ref ARGS: SayArgs = args::args();}

fn main() {
    let mut iter = ARGS.text.iter();
    let mut text = iter.next().unwrap_or(&EMPTY);
    loop {
        if ARGS.backslash_interpretation {
            print!("{}", interpretation(text))
        }else {
            print!("{}", text)
        }
        match iter.next() {
            Some(next) => {
                print!(" ");
                text = next
            }
            None => {
                print!("{}", if ARGS.trim_new_line { "" } else { "\n" });
                break
            }
        }
    }
}

fn interpretation(text: &String) -> String {
    let mut result = String::with_capacity(text.len());
    let mut is_escaped = false;
    for char in text.chars() {
        if is_escaped {
            match escaped(char) {
                Some(c) => {
                    is_escaped = false;
                    result.push(c);
                }
                None => {
                    print!("{}{}", result, if ARGS.trim_new_line { "" } else { "\n" })
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

fn escaped(c: char) -> Option<char> {
    Some(match c {
        '\\' => '\\',
        'a' => '\x07',
        'b' => '\x08',
        'e' => '\x1B',
        'f' => '\x0C',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        'v' => '\x0B',
        'c' => {
            return None;
        }
        _ => {
            eprintln!("\\{} is not a valid escape character, if you don't want to escape that try escaping \\ using \\\\{0}", c);
            exit(1)
        }
    })
}
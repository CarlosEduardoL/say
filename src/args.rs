use std::process::exit;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "say")]
#[clap(version = "v0.1.0")]
#[clap(author = "CarlosEduardoL")]
#[clap(about = "Rust echo copy", long_about = "If -e is in effect, the following sequences are recognized:
  \\\\     backslash
  \\a     alert (BEL)
  \\b     backspace
  \\c     produce no further output
  \\e     escape
  \\f     form feed
  \\n     new line
  \\r     carriage return
  \\t     horizontal tab
  \\v     vertical tab")]
struct _SayArgs {
    #[clap(value_name = "TEXT")]
    /// Text to be printed on the stdout
    text: Vec<String>,
    #[clap(short = 'n')]
    /// do not output the trailing newline
    trim_new_line: bool,
    #[clap(short = 'e')]
    /// enable interpretation of backslash escapes
    backslash_interpretation: bool,
    #[clap(short = 'E')]
    /// disable interpretation of backslash escapes (default)
    no_backslash_interpretation: bool,
}

#[derive(Debug)]
pub struct SayArgs {
    pub text: Vec<String>,
    pub backslash_interpretation: bool,
    pub trim_new_line: bool,
}

pub fn args() -> SayArgs {
    let args = _SayArgs::parse();
    if args.backslash_interpretation && args.no_backslash_interpretation {
        eprintln!("-e and -E are incompatibles flags");
        exit(1);
    }
    SayArgs {
        text: args.text,
        trim_new_line: args.trim_new_line,
        backslash_interpretation: args.backslash_interpretation,
    }
}
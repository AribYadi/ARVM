mod compiler;
mod parser;

use std::path::PathBuf;

use argh::FromArgs;
use chumsky::{
  Parser,
  Stream,
};
use logos::Logos;
use parser::Token;

#[derive(FromArgs)]
/// ARVM
struct Args {
  #[argh(positional)]
  /// input '.arvm' file
  input_file: PathBuf,
  #[argh(option)]
  /// output '.c' file
  output_file: Option<PathBuf>,
}

fn main() {
  let mut args: Args = argh::from_env();
  args.output_file.get_or_insert(args.input_file.with_extension("c"));

  let input_file = std::fs::read_to_string(args.input_file).unwrap();

  let lexer = Token::lexer(&input_file).spanned();
  let expr =
    parser::parser(&input_file).parse(Stream::from_iter(0..input_file.len(), lexer)).unwrap();
  let compiled_c = compiler::compiler(expr);

  let _ = std::fs::write(args.output_file.unwrap(), compiled_c);
}

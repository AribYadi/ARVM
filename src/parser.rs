use chumsky::prelude::*;
use logos::Logos;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Logos)]
enum Token {
  #[error]
  #[regex("[\n\t\r ]+", logos::skip)]
  Unknown,

  #[regex(r"\d+")]
  Integer,
}

#[derive(Debug, PartialEq)]
enum Expr {
  Integer(u64),
}

fn parser(src: &str) -> impl Parser<Token, Expr, Error = Simple<Token>> + '_ {
  just(Token::Integer).map_with_span(|_, sp| Expr::Integer(str::parse(&src[sp]).unwrap()))
}

mod tests {
  use chumsky::Stream;

  use super::*;

  fn parse(src: &str) -> Result<Expr, Vec<Simple<Token>>> {
    parser(src).parse(Stream::from_iter(0..src.len(), Token::lexer(src).spanned()))
  }

  #[test]
  fn parse_integer() { assert_eq!(parse("123"), Ok(Expr::Integer(123))) }
}

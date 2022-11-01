use std::fmt::Write;

use crate::parser::Expr;

pub fn compiler(expr: Expr) -> String {
  let mut compiled = r#"#include <stdio.h>

int main(void) {
"#
  .to_string();

  let _ = match expr {
    Expr::Integer(integer) => writeln!(compiled, r#"  printf("%d\n", {integer});"#),
  };

  compiled.push('}');
  compiled
}

mod tests {
  use super::*;
  use crate::parser::*;
  use chumsky::{
    Parser,
    Stream,
  };
  use logos::Logos;

  fn compile(src: &str) -> String {
    compiler(
      parser(src).parse(Stream::from_iter(0..src.len(), Token::lexer(src).spanned())).unwrap(),
    )
  }

  #[test]
  fn compile_integer() {
    assert_eq!(
      compile("123"),
      r#"#include <stdio.h>

int main(void) {
  printf("%d\n", 123);
}"#
    )
  }
}

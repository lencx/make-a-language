use crate::utils;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
  pub fn new(s: &str) -> (&str, Self) {
    let (s, number) = utils::extract_digits(s);
    (s, Self(number.parse().unwrap()))
  }
}

#[derive(Debug, PartialEq)]
pub enum Op {
  Add,
  Sub,
  Mul,
  Div,
  Pow,
}

impl Op {
  pub fn new(s: &str) -> (&str, Self) {
    let (s, op) = utils::extract_op(s);
    let op = match op {
      "+" => Self::Add,
      "-" => Self::Sub,
      "*" => Self::Mul,
      "/" => Self::Div,
      "^" => Self::Pow,
      _ => unreachable!(),
    };

    (s, op)
  }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
  pub lhs: Number,
  pub rhs: Number,
  pub op: Op,
}

impl Expr {
  pub fn new(s: &str) -> (&str, Self) {
    let (s, lhs) = Number::new(s);
    let (s, _) = utils::extract_space(s);
    let (s, op) = Op::new(s);
    let (s, _) = utils::extract_space(s);
    let (s, rhs) = Number::new(s);

    (s, Self { lhs, rhs, op })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_number() {
    assert_eq!(Number::new("1234"), ("", Number(1234)));
  }

  #[test]
  fn parse_op() {
    assert_eq!(Op::new("+"), ("", Op::Add));
    assert_eq!(Op::new("-"), ("", Op::Sub));
    assert_eq!(Op::new("*"), ("", Op::Mul));
    assert_eq!(Op::new("/"), ("", Op::Div));
    assert_eq!(Op::new("^"), ("", Op::Pow));
  }

  #[test]
  fn parse_one_plus_two() {
    assert_eq!(
      Expr::new("1+2"),
      (
        "",
        Expr {
          lhs: Number(1),
          rhs: Number(2),
          op: Op::Add,
        }
      )
    )
  }

  #[test]
  fn parse_expr_with_whitespace() {
    assert_eq!(
      Expr::new("2 * 4"),
      (
        "",
        Expr {
          lhs: Number(2),
          rhs: Number(4),
          op: Op::Mul,
        }
      )
    )
  }
}
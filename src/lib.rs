mod utils;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
  pub fn new(s: &str) -> Self {
    Self(s.parse().unwrap())
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
  pub fn new(s: &str) -> Self {
    match s {
      "+" => Self::Add,
      "-" => Self::Sub,
      "*" => Self::Mul,
      "/" => Self::Div,
      "^" => Self::Pow,
      _ => panic!("Invalid operator"),
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
  pub lhs: Number,
  pub rhs: Number,
  pub op: Op,
}

impl Expr {
  pub fn new(s: &str) -> Self {
    // let lhs = Number::new(s);
    // let rhs = Number::new(s);
    // let op = Op::new(s);

    let (s, lhs) = utils::extract_digits(s);
    let lhs = Number::new(lhs);

    let (s, op) = utils::extract_op(s);
    let op = Op::new(op);

    let (_s, rhs) = utils::extract_digits(s);
    let rhs = Number::new(rhs);

    Self { lhs, rhs, op }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_number() {
    assert_eq!(Number::new("1234"), Number(1234));
  }

  #[test]
  fn parse_op() {
    assert_eq!(Op::new("+"), Op::Add);
    assert_eq!(Op::new("-"), Op::Sub);
    assert_eq!(Op::new("*"), Op::Mul);
    assert_eq!(Op::new("/"), Op::Div);
    assert_eq!(Op::new("^"), Op::Pow);
  }

  // #[test]
  // fn parse_one_plus_two() {
  //   assert_eq!(
  //     Expr::new("1+2"),
  //     Expr {
  //       lhs: Number(1),
  //       rhs: Number(2),
  //       op: Op::Add,
  //     }
  //   )
  // }
}
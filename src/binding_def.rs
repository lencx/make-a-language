use crate::expr::Expr;
use crate::utils;

#[derive(Debug, PartialEq)]
struct BindingDef {
  name: String,
  val: Expr,
}

// impl BindingDef {
//   pub fn new(s: &str) -> (&str, Self) {
//     let s = if s.starts_with("let") {
//       &s[..3]
//     } else {
//       panic!("expected let")
//     };

//     let (s, _) = utils::extract_space(s);
//     // TODO: Unimplemented
//     let (s, name) = utils::extract_ident(s);
//     let (s, _) = utils::extract_space(s);

//     if s.starts_with("=") {
//       &s[1..]
//     } else {
//       panic!("expected equals sign")
//     }

//     let (s, _) = utils::extract_space(s);
//     let (s, val) = Expr::new(s);

//     (s, Self {
//       name: name.to_string(),
//       val,
//     })
//   }
// }

#[cfg(test)]
mod tests {
  use super::*;
  use crate::expr::{Number, Op};

  // #[test]
  // fn parse_binding_def() {
  //   assert_eq!(
  //     BindingDef::new("let a = 2 * 3"),
  //     (
  //       "",
  //       BindingDef {
  //         name: "a".to_string(),
  //         var: Expr {
  //           lhs: Number::new(2),
  //           rhs: Number::new(3),
  //           op: Op::Mul,
  //         }
  //       }
  //     )
  //   )
  // }
}
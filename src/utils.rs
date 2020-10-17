// pub(crate) makes an item visible within the current crate.
// @see: https://doc.rust-lang.org/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself
#[allow(dead_code)]
pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
  // (&s[1..], &s[0..1])
  // TODO: float
  // let digits_end = s.char_indices()
  //   .find_map(|(idx, c)| {
  //     let temp = (c == '_' || c == '.') && s.is_char_boundary(idx+1);
  //     if c.is_ascii_digit() || temp { None }
  //     else { Some(idx) }
  //   }).unwrap_or_else(|| s.len());

  // let digits = &s[..digits_end];
  // let remainder = &s[digits_end..];
  // (remainder, digits)
    take_while(|c, idx| {
      let temp = (c == '_' || c == '.') && s.is_char_boundary(idx+1);
      c.is_ascii_digit() || temp
    }, s)
}

#[allow(dead_code)]
pub(crate) fn extract_op(s: &str) -> (&str, &str) {
  match &s[0..1] {
    "+" | "-" | "*" | "/" | "^" => {},
    _ => panic!("Invalid operator"),
  }

  (&s[1..], &s[0..1])
}

#[allow(dead_code)]
pub(crate) fn extract_space(s: &str) -> (&str, &str) {
  // let space_end = s.char_indices()
  //   .find_map(|(idx, c)| {
  //     if c == ' ' { None }
  //     else { Some(idx) }
  //   }).unwrap_or_else(|| s.len());

  // let space = &s[..space_end];
  // let remainder = &s[space_end..];
  // (remainder, space)
  take_while(|c, _| c == ' ', s)
}

pub(crate) fn extract_ident(s: &str) -> (&str, &str) {
  let starts_with_alphabetic = s.chars()
    .next()
    .map(|c| c.is_ascii_alphabetic())
    .unwrap_or(false);

  if starts_with_alphabetic {
    take_while(|c, _| c.is_ascii_alphanumeric(), s)
  } else {
    (s, "")
  }
}

pub fn take_while(accept: impl Fn(char, usize) -> bool, s: &str) -> (&str, &str) {
  let extracted_end = s.char_indices()
    .find_map(|(idx, c)| if accept(c, idx) { None } else { Some(idx) })
    .unwrap_or_else(|| s.len());

  let extracted = &s[..extracted_end];
  let remainder = &s[extracted_end..];
  (remainder, extracted)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn extract_one_digits() {
    assert_eq!(extract_digits("1+2"), ("+2", "1"));
  }

  #[test]
  fn do_not_extract_anything_from_empty_input() {
    assert_eq!(extract_digits(""), ("", ""))
  }

  #[test]
  fn extract_digits_with_no_remainder() {
    assert_eq!(extract_digits("100"), ("", "100"))
  }

  #[test]
  fn extract_multiple_digits() {
    assert_eq!(extract_digits("15+20"), ("+20", "15"));
  }

  #[test]
  fn extract_underline_digits() {
    assert_eq!(extract_digits("15_000+20"), ("+20", "15_000"));
  }

  #[test]
  fn extract_operator() {
    assert_eq!(extract_op("+2"), ("2", "+"));
    assert_eq!(extract_op("-2"), ("2", "-"));
    assert_eq!(extract_op("*2"), ("2", "*"));
    assert_eq!(extract_op("/2"), ("2", "/"));
  }

  #[test]
  fn extract_spaces() {
    assert_eq!(extract_space("   1"), ("1", "   "));
  }

  #[test]
  fn extract_alphabetic_ident() {
    assert_eq!(extract_ident("testAbc skas"), (" skas", "testAbc"));
  }

  #[test]
  fn extract_alphanumeric_ident() {
    assert_eq!(extract_ident("test123 skas"), (" skas", "test123"));
    assert_eq!(extract_ident("123test skas"), ("123test skas", ""));
  }
}
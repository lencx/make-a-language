// pub(crate) makes an item visible within the current crate.
// @see: https://doc.rust-lang.org/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself
pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
  // (&s[1..], &s[0..1])
  // TODO: float
  let digits_end = s.char_indices()
    .find_map(|(idx, c)| {
      if (c.is_ascii_digit()) { None }
      else { Some(idx) }
    }).unwrap_or_else(|| s.len());

  let digits = &s[..digits_end];
  let remainder = &s[digits_end..];
  (remainder, digits)
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn extract_one_digits() {
  //   assert_eq!(extract_digits("1+2"), ("+2", "12"));
  // }

  #[test]
  fn extract_multiple_digits() {
    assert_eq!(extract_digits("15+20"), ("+20", "15"));
  }
}
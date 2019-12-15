#[cfg(test)]
mod tests {
  use crate::strformat;

  #[test]
  fn fmt() {
    let fmt = "{2} {1} {3} {0}".to_string();
    let input = ["Test", "is", "This", "a"];
    let result = "This is a Test";
    assert_eq!(strformat::fmt(fmt, &input), result);
  }
}
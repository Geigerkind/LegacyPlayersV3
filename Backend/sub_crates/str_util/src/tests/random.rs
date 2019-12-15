#[cfg(test)]
mod tests {
  use crate::random;

  #[test]
  fn alphanumeric() {
    let result = random::alphanumeric(42);
    assert_eq!(result.len(), 42);
  }
}
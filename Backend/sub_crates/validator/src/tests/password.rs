#[cfg(test)]
mod tests {
  extern crate dotenv;
  use crate::tools::valid_password;
  use self::dotenv::dotenv;

  #[test]
  fn password_too_short() {
    dotenv().ok();
    let pass = "tooshort";
    assert!(valid_password(pass).is_err());
  }

  #[test]
  fn password_has_been_pwned() {
    dotenv().ok();
    let pass = "Password123456";
    assert!(valid_password(pass).is_err());
  }

  #[test]
  fn password_is_secure_enough() {
    dotenv().ok();
    let pass = "Password123456Password123456Password123456";
    assert!(valid_password(pass).is_ok());
  }
}
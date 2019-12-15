#[cfg(test)]
mod tests {
  use crate::modules::account::material::Account;
  use crate::modules::account::tools::Login;

  // User exists login is tested when creating an account
  #[test]
  fn login_user_does_not_exist() {
    let account = Account::default();
    let login = account.login("NothingLol", "NotSecret");
    assert!(login.is_err());
  }
}
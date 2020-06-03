use crate::modules::account::{material::Account, tools::Login};
use crate::start_test_db;

// User exists login is tested when creating an account
#[test]
fn login_user_does_not_exist() {
    let dns: String;
    start_test_db!(false, dns);

    let account = Account::with_dns((dns + "main").as_str());
    let login = account.login("NothingLol", "NotSecret");
    assert!(login.is_err());
}

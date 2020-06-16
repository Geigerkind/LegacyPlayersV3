use crate::modules::account::{material::Account, tools::Login};
use crate::tests::TestContainer;

// User exists login is tested when creating an account
#[test]
fn login_user_does_not_exist() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    let account = Account::with_dns((dns + "main").as_str());
    let login = account.login("NothingLol", "NotSecret");
    assert!(login.is_err());
}

use crate::modules::account::{material::Account, tools::Login};
use crate::tests::TestContainer;

// User exists login is tested when creating an account
#[test]
fn login_user_does_not_exist() {
    let container = TestContainer::new(false);
    let (mut conn, _dns, _node) = container.run();

    let account = Account::default();
    let login = account.login(&mut conn, "NothingLol", "NotSecret");
    assert!(login.is_err());
}

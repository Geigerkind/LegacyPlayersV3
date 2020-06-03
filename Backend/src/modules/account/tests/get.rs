use crate::modules::account::{
    material::Account,
    tools::{Create, GetAccountInformation},
};
use crate::start_test_db;
use crate::modules::account::tests::helper::get_create_member;

#[test]
fn get_does_not_exist() {
    let dns: String;
    start_test_db!(false, dns);

    let account = Account::with_dns((dns + "main").as_str());
    let acc_info = account.get(42);
    assert!(acc_info.is_err());
}

#[test]
fn get_exists() {
    let dns: String;
    start_test_db!(false, dns);

    let account = Account::with_dns((dns + "main").as_str());
    let post_obj = get_create_member("abc", "abc@abc.de", "Password123456Password123456Password123456");

    let login = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let acc_info = account.get(login.member_id);
    assert!(acc_info.is_ok());
}

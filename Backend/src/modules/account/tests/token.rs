use crate::modules::account::tests::helper::get_create_member;
use crate::modules::account::{
    material::Account,
    tools::{Create, Login, Token, Update},
};
use crate::tests::TestContainer;
use str_util::sha3;

#[test]
fn validate_valid() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    let account = Account::with_dns((dns + "main").as_str());
    let post_obj = get_create_member("abc", "abc@abc.de", "Password123456Password123456Password123456");

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    assert!(account.validate_token(api_token.token.as_ref().unwrap()).is_some());
}

#[test]
fn validate_invalid() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    let account = Account::with_dns((dns + "main").as_str());
    assert!(account.validate_token("someHash").is_none());
}

#[test]
fn validation_invalid_after_update() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    let account = Account::with_dns((dns + "main").as_str());
    let post_obj = get_create_member("abc", "abc@abc.de", "Password123456Password123456Password123456");

    // First login
    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let api_token_two = account.login(&post_obj.credentials.mail, &post_obj.credentials.password).unwrap();
    assert!(account.validate_token(api_token.token.as_ref().unwrap()).is_some());
    assert!(account.validate_token(api_token_two.token.as_ref().unwrap()).is_some());

    let api_token_three = account.change_password("SuperDuperSecretPasswordDefNotSecretTho", api_token.member_id).unwrap();
    assert!(account.validate_token(api_token_two.token.as_ref().unwrap()).is_none());
    assert!(account.validate_token(api_token_three.token.as_ref().unwrap()).is_some());
}

#[test]
fn get_all_tokens() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    let account = Account::with_dns((dns + "main").as_str());
    let post_obj = get_create_member("abc", "abc@abc.de", "Password123456Password123456Password123456");

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let tokens = account.get_all_token(api_token.member_id);
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token, None);
}

#[test]
fn delete_token() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    let account = Account::with_dns((dns + "main").as_str());
    let post_obj = get_create_member("abc", "abc@abc.de", "Password123456Password123456Password123456");

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    assert!(account.validate_token(&api_token.token.as_ref().unwrap()).is_some());

    let new_token_res = account.create_token("Login", api_token.member_id, time_util::get_ts_from_now_in_secs(7));
    assert!(new_token_res.is_ok());
    let new_token = new_token_res.unwrap();
    assert!(account.validate_token(new_token.token.as_ref().unwrap()).is_some());

    assert!(account.delete_token(new_token.id, api_token.member_id).is_ok());
    assert!(account.validate_token(new_token.token.as_ref().unwrap()).is_none());
}

#[test]
fn prolong_token() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    let account = Account::with_dns((dns + "main").as_str());
    let post_obj = get_create_member("abc", "abc@abc.de", "Password123456Password123456Password123456");

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let in_seven_days = time_util::get_ts_from_now_in_secs(7);
    assert!(in_seven_days - api_token.exp_date <= 5);

    let db_token = sha3::hash(&[&api_token.token.as_ref().unwrap().clone(), &"token".to_owned()]);
    let member_id = *account.api_token_to_member_id.read().unwrap().get(&db_token).unwrap();
    let new_token = account.prolong_token(api_token.id, member_id, 30);
    assert!(new_token.is_ok());
    let in_thirty_days = time_util::get_ts_from_now_in_secs(30);
    assert!(in_thirty_days - new_token.unwrap().exp_date <= 5);
}

#[test]
fn prolong_token_by_str() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    let account = Account::with_dns((dns + "main").as_str());
    let post_obj = get_create_member("abc", "abc@abc.de", "Password123456Password123456Password123456");

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let in_seven_days = time_util::get_ts_from_now_in_secs(7);
    assert!(in_seven_days - api_token.exp_date <= 5);

    let db_token = sha3::hash(&[&api_token.token.as_ref().unwrap().clone(), &"token".to_owned()]);
    let member_id = *account.api_token_to_member_id.read().unwrap().get(&db_token).unwrap();
    let new_token = account.prolong_token_by_str(api_token.token.as_ref().unwrap().clone(), member_id, 30);
    assert!(new_token.is_ok());
    let in_thirty_days = time_util::get_ts_from_now_in_secs(30);
    assert!(in_thirty_days - new_token.unwrap().exp_date <= 5);
}

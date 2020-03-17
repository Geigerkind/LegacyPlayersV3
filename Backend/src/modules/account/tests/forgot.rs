use mysql_connection::tools::Execute;
use str_util::sha3;

use crate::modules::account::{
    dto::{CreateMember, Credentials},
    material::Account,
    tools::{Create, Forgot},
};

#[test]
fn send_forget_password_user_does_not_exist() {
    let account = Account::default();
    assert!(account.send_forgot_password("test@mail.de").is_ok());
}

#[test]
fn send_forget_password_invalid_mail() {
    let account = Account::default();
    assert!(account.send_forgot_password("test").is_err());
}

#[test]
fn send_forgot_password_user_exists_and_receive() {
    let account = Account::default();
    let post_obj = CreateMember {
        nickname: "fscngsuzfdcsv".to_string(),
        credentials: Credentials {
            mail: "fscngsuzfdcsv@jaylappTest.dev".to_string(),
            password: "Password123456Password123456Password123456".to_string(),
        },
    };

    let val_pair = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    assert!(account.send_forgot_password("fscngsuzfdcsv@jaylappTest.dev").is_ok());

    let salt;
    {
        let member = account.member.read().unwrap();
        salt = member.get(&val_pair.member_id).unwrap().salt.clone();
    }
    let forgot_id = sha3::hash(&[&val_pair.member_id.to_string(), "forgot", &salt]);
    let receive_forgot = account.recv_forgot_password(&forgot_id);
    assert!(receive_forgot.is_ok());

    account.db_main.execute("DELETE FROM account_member WHERE mail='fscngsuzfdcsv@jaylappTest.dev'");
}

#[test]
fn recv_forgot_password_invalid_id() {
    let account = Account::default();
    assert!(account.recv_forgot_password("bla").is_err());
}

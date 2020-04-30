use crate::modules::account::Account;
use crate::modules::account::dto::{CreateMember, Credentials};
use crate::modules::account::tools::Create;
use mysql_connection::tools::Execute;
use proptest::prelude::*;

proptest! {
  #[test]
  fn create_account_with_random_params_should_not_crash(nickname in "\\PC*", password in "\\PC*", mail_prefix in "\\PC*") {
    // Arrange
    let account = Account::default();
    let acc_mail = mail_prefix + "mail_rt@jaylappTest.dev";
    let post_obj = CreateMember {
      nickname,
      credentials: Credentials {
        mail: acc_mail.to_string(),
        password,
      },
    };

    // Act + Assert
    let _ = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password);

    // Cleanup
    account.db_main.execute_wparams("DELETE FROM account_member WHERE mail=:mail", params!("mail" => acc_mail));
  }
}
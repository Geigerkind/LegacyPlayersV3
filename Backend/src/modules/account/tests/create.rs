#[cfg(test)]
mod tests {
  use mysql_connection::tools::Execute;
  use str_util::sha3;

  use crate::modules::account::dto::{CreateMember, Credentials};
  use crate::modules::account::material::Account;
  use crate::modules::account::tools::{Create, GetAccountInformation};

  #[test]
  fn create_account() {
    let account = Account::default();
    let acc_mail = "mail@jaylappTest.dev";
    let post_obj = CreateMember {
      nickname: "NickName".to_string(),
      credentials: Credentials {
        mail: acc_mail.to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let login = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password);
    assert!(login.is_ok());

    account.db_main.execute("DELETE FROM member WHERE mail='mail@jaylappTest.dev'");
  }

  #[test]
  fn mail_twice() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "BlaNameqqweq".to_string(),
      credentials: Credentials {
        mail: "bla@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let _ = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    assert!(account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='bla@jaylappTest.dev'");
  }

  #[test]
  fn nickname_twice() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "BlaName".to_string(),
      credentials: Credentials {
        mail: "bla2@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let post_obj_two = CreateMember {
      nickname: "BlaName".to_string(),
      credentials: Credentials {
        mail: "bla3@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let _ = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    assert!(account.create(&post_obj_two.credentials.mail, &post_obj_two.nickname, &post_obj_two.credentials.password).is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='bla2@jaylappTest.dev'");
    account.db_main.execute("DELETE FROM member WHERE mail='bla3@jaylappTest.dev'");
  }

  #[test]
  fn mail_empty() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "NickName".to_string(),
      credentials: Credentials {
        mail: "".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    assert!(account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).is_err());
  }

  #[test]
  fn password_empty() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "NickName".to_string(),
      credentials: Credentials {
        mail: "34234234".to_string(),
        password: "".to_string(),
      },
    };

    assert!(account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).is_err());
  }

  #[test]
  fn nickname_empty() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "".to_string(),
      credentials: Credentials {
        mail: "34234234".to_string(),
        password: "dgsdfsfd".to_string(),
      },
    };

    assert!(account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).is_err());
  }

  #[test]
  fn invalid_mail() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "asdasd".to_string(),
      credentials: Credentials {
        mail: "34234234".to_string(),
        password: "dgsdfsfd".to_string(),
      },
    };

    assert!(account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).is_err());
  }

  #[test]
  fn invalid_nickname() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "asdasd asdfsdfs".to_string(),
      credentials: Credentials {
        mail: "abc@test.de".to_string(),
        password: "dgsdfsfd".to_string(),
      },
    };

    assert!(account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).is_err());
  }

  #[test]
  fn confirm_mail() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "SomeNameWuuh".to_string(),
      credentials: Credentials {
        mail: "someNameWuuuuh@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let login = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let mail_id;
    {
      let member_guard = account.member.read().unwrap();
      let member = member_guard.get(&login.member_id).unwrap();
      mail_id = sha3::hash(&[&login.member_id.to_string(), "mail", &member.salt]);
    }
    account.confirm(&mail_id);
    let confirmed_information = account.get(login.member_id).unwrap();
    assert!(confirmed_information.mail_confirmed);

    account.db_main.execute("DELETE FROM member WHERE mail='someNameWuuuuh@jaylappTest.dev'");
  }
}
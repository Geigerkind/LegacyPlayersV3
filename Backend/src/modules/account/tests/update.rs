#[cfg(test)]
mod tests {
  use mysql_connection::tools::Execute;
  use str_util::sha3;

  use crate::modules::account::dto::{CreateMember, Credentials};
  use crate::modules::account::material::Account;
  use crate::modules::account::tools::{Create, Update};

  #[test]
  fn change_name() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "ijofsdiojsdfgiuhig".to_string(),
      credentials: Credentials {
        mail: "ijofsdiojsdfgiuhig@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let changed_name = account.change_name("SomeUsername", api_token.member_id);
    assert!(changed_name.is_ok());
    assert_eq!(changed_name.unwrap().nickname, "SomeUsername".to_string());

    account.db_main.execute("DELETE FROM member WHERE mail='ijofsdiojsdfgiuhig@jaylappTest.dev'");
  }

  #[test]
  fn change_name_empty_content() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "siodjfijsiojiospq".to_string(),
      credentials: Credentials {
        mail: "siodjfijsiojiospq@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let changed_name = account.change_name("", api_token.member_id);
    assert!(changed_name.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='siodjfijsiojiospq@jaylappTest.dev'");
  }

  #[test]
  fn change_name_invalid_content() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "ihsdfoiosdf".to_string(),
      credentials: Credentials {
        mail: "ihsdfoiosdf@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let changed_name = account.change_name("ihsdfoiosdf ihsdfoiosdf", api_token.member_id);
    println!("{:?}", changed_name);
    assert!(changed_name.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='ihsdfoiosdf@jaylappTest.dev'");
  }

  #[test]
  fn change_name_name_taken() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "oasijidhaais".to_string(),
      credentials: Credentials {
        mail: "oasijidhaais@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let post_obj_two = CreateMember {
      nickname: "guhzasooas".to_string(),
      credentials: Credentials {
        mail: "guhzasooas@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let _ = account.create(&post_obj_two.credentials.mail, &post_obj_two.nickname, &post_obj_two.credentials.password).unwrap();
    let changed_name = account.change_name(&post_obj_two.nickname, api_token.member_id);
    assert!(changed_name.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='oasijidhaais@jaylappTest.dev'");
    account.db_main.execute("DELETE FROM member WHERE mail='guhzasooas@jaylappTest.dev'");
  }

  #[test]
  fn change_password_empty_content() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "mvfhhbvidsd".to_string(),
      credentials: Credentials {
        mail: "mvfhhbvidsd@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let changed_password = account.change_password("", api_token.member_id);
    assert!(changed_password.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='mvfhhbvidsd@jaylappTest.dev'");
  }

  #[test]
  fn change_password() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "xdsdfgsdgs".to_string(),
      credentials: Credentials {
        mail: "xdsdfgsdgs@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let changed_password = account.change_password("SomeWeirdPassword", api_token.member_id);
    assert!(changed_password.is_ok());
    let new_api_token = changed_password.unwrap();
    assert_ne!(new_api_token.token, api_token.token);
    assert_ne!(new_api_token.id, api_token.id);
    assert_eq!(new_api_token.member_id, api_token.member_id);

    account.db_main.execute("DELETE FROM member WHERE mail='xdsdfgsdgs@jaylappTest.dev'");
  }

  #[test]
  fn change_mail_empty_content() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "nsigsvbsdsd".to_string(),
      credentials: Credentials {
        mail: "nsigsvbsdsd@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let changed_mail = account.request_change_mail("", api_token.member_id);
    assert!(changed_mail.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='nsigsvbsdsd@jaylappTest.dev'");
  }

  #[test]
  fn change_mail_invalid_content() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "asiudfuhisduifs".to_string(),
      credentials: Credentials {
        mail: "asiudfuhisduifs@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let changed_mail = account.request_change_mail("asiudfuhisduifs", api_token.member_id);
    assert!(changed_mail.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='asiudfuhisduifs@jaylappTest.dev'");
  }

  #[test]
  fn change_mail_mail_taken() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "csdazgtsdczas".to_string(),
      credentials: Credentials {
        mail: "csdazgtsdczas@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let post_obj_two = CreateMember {
      nickname: "bdvshudvbsdv".to_string(),
      credentials: Credentials {
        mail: "bdvshudvbsdv@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();
    let _ = account.create(&post_obj_two.credentials.mail, &post_obj_two.nickname, &post_obj_two.credentials.password).unwrap();
    let changed_mail = account.request_change_mail(&post_obj_two.credentials.mail, api_token.member_id);
    assert!(changed_mail.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='csdazgtsdczas@jaylappTest.dev'");
    account.db_main.execute("DELETE FROM member WHERE mail='bdvshudvbsdv@jaylappTest.dev'");
  }

  #[test]
  fn change_mail() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "xdssdfsdfg".to_string(),
      credentials: Credentials {
        mail: "xdssdfsdfg@jaylappTest.dev".to_string(),
        password: "Password123456Password123456Password123456".to_string(),
      },
    };

    let api_token = account.create(&post_obj.credentials.mail, &post_obj.nickname, &post_obj.credentials.password).unwrap();

    let salt;
    {
      let mut member = account.member.write().unwrap();
      let mut member_entry = member.get_mut(&api_token.member_id).unwrap();
      member_entry.mail_confirmed = true;
      salt = member_entry.salt.clone();
    }

    let request_change_mail = account.request_change_mail("xdssdfsdfg2@bla.de", api_token.member_id);
    assert!(request_change_mail.is_ok());
    let confirm_id = sha3::hash(&[&api_token.member_id.to_string(), "new_mail", &salt]);
    let changed_mail = account.confirm_change_mail(&confirm_id);
    assert!(changed_mail.is_ok());

    let new_api_token = changed_mail.unwrap();
    assert_ne!(new_api_token.token, api_token.token);
    assert_ne!(new_api_token.id, api_token.id);
    assert_eq!(new_api_token.member_id, api_token.member_id);

    account.db_main.execute("DELETE FROM member WHERE mail='xdssdfsdfg2@bla.de'");
  }
}
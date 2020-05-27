use rocket::local::Client;
use rocket::http::{ContentType, Status};
use crate::modules::account::dto::{CreateMember, Credentials};
use crate::modules::account::Account;
use mysql_connection::tools::{Exists, Execute};

#[test]
fn create_account_nick_valid_email_valid_password_valid() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    // An http client is created, which will
    // be used to send http requests to the account creation endpoint
    let http_client = Client::new(rocket).expect("valid rocket instance");
    // An object that contains the input parameters is defined
    let post_obj = CreateMember {
        nickname: "someNickname".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    // Serialize the object to the json format
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::default();
    // Verify that no account with this email is known
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    // Http request is send to the endpoint, response is received
    let response = req.dispatch();

    // Then
    // Verify the status code of the response
    assert_eq!(response.status(), Status::Ok);
    // Verify that the user has been created in the database
    assert!(account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // Clean up
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}

#[test]
fn create_account_nick_used_email_valid_password_valid() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "someNickname".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let post_obj_used= CreateMember {
        nickname: "someNickname".to_string(),
        credentials: Credentials {
            mail: "someOtherEmail@someDomain.test".to_string(),
            password: "someOtherExtremelySecurePassword".to_string(),
        },
    };
    let json_body_used = serde_json::to_string(&post_obj_used).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj_used.credentials.mail)));
    // Create an account for this nickname
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req_used = http_client.post("/create")
        .header(ContentType::JSON).body(json_body_used.as_str());
    let response_used = req_used.dispatch();

    // Then
    assert_eq!(response_used.status(), Status::new(526, "NicknameIsInUse"));
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj_used.credentials.mail)));

    // Clean up
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj_used.credentials.mail));
}

#[test]
fn create_account_nick_malformed_email_valid_password_valid() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "some malformed nickname".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(522, "InvalidNickname"));
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // Clean up
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}

#[test]
fn create_account_nick_valid_email_used_password_valid() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "someNickname".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let post_obj_used = CreateMember {
        nickname: "someOtherNickname".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body_used = serde_json::to_string(&post_obj_used).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));
    // Create an account for this email
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req_used = http_client.post("/create")
        .header(ContentType::JSON).body(json_body_used.as_str());
    let response_used = req_used.dispatch();

    // Then
    assert_eq!(response_used.status(), Status::new(525, "MailIsInUse"));

    // Clean up
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}

#[test]
fn create_account_nick_valid_email_malformed_password_valid() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "someNickname".to_string(),
        credentials: Credentials {
            mail: "someMalformedEmail".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(521, "InvalidMail"));
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // Clean up (just in case)
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}

#[test]
fn create_account_nick_valid_email_valid_password_too_short() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "someNickname".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "tooShort".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(524, "PasswordTooShort"));
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // Clean up (just in case)
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}

#[test]
fn create_account_nick_valid_email_valid_password_pwned() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "someNickname".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "correcthorsebatterystaple".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(523, "PwnedPassword"));
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // Clean up (just in case)
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}

#[test]
fn create_account_nick_malformed_email_malformed_password_valid() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "some malformed nickname".to_string(),
        credentials: Credentials {
            mail: "someMalformedEmail".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(521, "InvalidMail"));
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // Clean up
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}

#[test]
fn create_account_nick_malformed_email_valid_password_too_short() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "some malformed nickname".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "tooShort".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(522, "InvalidNickname"));
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // Clean up
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}

#[test]
fn create_account_nick_malformed_email_valid_password_pwned() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "some malformed nickname".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "correcthorsebatterystaple".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(522, "InvalidNickname"));
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // Clean up
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}

#[test]
fn create_account_nick_valid_email_malformed_password_too_short() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "someNickname".to_string(),
        credentials: Credentials {
            mail: "someMalformedEmail".to_string(),
            password: "tooShort".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(521, "InvalidMail"));
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // Clean up
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}

#[test]
fn create_account_nick_valid_email_malformed_password_pwned() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "someNickname".to_string(),
        credentials: Credentials {
            mail: "someMalformedEmail".to_string(),
            password: "correcthorsebatterystaple".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(521, "InvalidMail"));
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // Clean up
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}

#[test]
fn create_account_nick_malformed_email_malformed_password_too_short() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "some malformed nickname".to_string(),
        credentials: Credentials {
            mail: "someMalformedEmail".to_string(),
            password: "tooShort".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(521, "InvalidMail"));
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // Clean up
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}

#[test]
fn create_account_nick_malformed_email_malformed_password_pwned() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "some malformed nickname".to_string(),
        credentials: Credentials {
            mail: "someMalformedEmail".to_string(),
            password: "correcthorsebatterystaple".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(521, "InvalidMail"));
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // Clean up
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}


#[test]
fn create_account_nick_used_email_used_password_valid() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "someNickname".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let post_obj_used = CreateMember {
        nickname: "someNickname".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body_used = serde_json::to_string(&post_obj_used).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));
    // Create an account for this user
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req_used = http_client.post("/create")
        .header(ContentType::JSON).body(json_body_used.as_str());
    let response_used = req_used.dispatch();

    // Then
    assert_eq!(response_used.status(), Status::new(525, "MailIsInUse"));

    // Clean up
    account.db_main.execute(&format!(
        "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}

#[test]
fn create_account_nick_malformed_email_used_password_valid() {
    // Given
    let account = Account::default();
    let rocket = rocket::ignite().manage(account)
        .mount("/", routes![crate::modules::account::transfer::create::create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "someNickname".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let post_obj_used = CreateMember {
        nickname: "some malformed nickname".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body_used = serde_json::to_string(&post_obj_used).unwrap();
    let account = Account::default();
    assert!(!account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));
    // Create an account for this email
    let req = http_client.post("/create")
        .header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(account.db_main.exists(&format!(
        "SELECT * FROM account_member WHERE mail='{}'", post_obj.credentials.mail)));

    // When
    let req_used = http_client.post("/create")
        .header(ContentType::JSON).body(json_body_used.as_str());
    let response_used = req_used.dispatch();

    // Then
    assert_eq!(response_used.status(), Status::new(522, "InvalidNickname"));

    // Clean up
    account.db_main.execute(&format!(
    "DELETE FROM account_member WHERE mail='{}'", post_obj.credentials.mail));
}


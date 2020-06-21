use crate::modules::account::dto::{CreateMember, Credentials};
use crate::modules::account::Account;
use crate::tests::TestContainer;
use mysql_connection::tools::Exists;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

fn has_existing_entry(account: &Account, email: &str) -> bool {
    account.db_main.exists(&format!("SELECT * FROM account_member WHERE mail='{}'", email))
}

fn create_http_client(dns: String) -> Client {
    let account = Account::with_dns(&dns).init();
    let rocket = rocket::ignite().manage(account).mount("/", routes![crate::modules::account::transfer::create::create]);
    Client::new(rocket).expect("valid rocket instance")
}

#[test]
fn valid_account_creation() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    // Given
    let http_client = create_http_client(dns.clone());
    // An object that contains the input parameters is defined
    let post_obj = CreateMember {
        nickname: "someNickname1".to_string(),
        credentials: Credentials {
            mail: "someEmail1@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    // Serialize the object to the json format
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::with_dns(&dns);
    // Verify that no account with this email is known
    assert!(!has_existing_entry(&account, &post_obj.credentials.mail));

    // When
    let req = http_client.post("/create").header(ContentType::JSON).body(json_body.as_str());
    // Http request is send to the endpoint, response is received
    let response = req.dispatch();

    // Then
    // Verify the status code of the response
    assert_eq!(response.status(), Status::Ok);
    // Verify that the user has been created in the database
    assert!(has_existing_entry(&account, &post_obj.credentials.mail));
}

#[test]
fn nickname_in_use() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    // Given
    let http_client = create_http_client(dns.clone());
    let post_obj = CreateMember {
        nickname: "someNickname2".to_string(),
        credentials: Credentials {
            mail: "someEmail2@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let post_obj_used = CreateMember {
        nickname: "someNickname2".to_string(),
        credentials: Credentials {
            mail: "someOtherEmail2@someDomain.test".to_string(),
            password: "someOtherExtremelySecurePassword".to_string(),
        },
    };
    let json_body_used = serde_json::to_string(&post_obj_used).unwrap();
    let account = Account::with_dns(&dns);
    assert!(!has_existing_entry(&account, &post_obj.credentials.mail));
    assert!(!has_existing_entry(&account, &post_obj_used.credentials.mail));
    // Create an account for this nickname
    let req = http_client.post("/create").header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(has_existing_entry(&account, &post_obj.credentials.mail));

    // When
    let req_used = http_client.post("/create").header(ContentType::JSON).body(json_body_used.as_str());
    let response_used = req_used.dispatch();

    // Then
    assert_eq!(response_used.status(), Status::new(526, "NicknameIsInUse"));
    assert!(!has_existing_entry(&account, &post_obj_used.credentials.mail));
}

#[test]
fn nickname_malformed() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    // Given
    let http_client = create_http_client(dns.clone());
    let post_obj = CreateMember {
        nickname: "some malformed nickname3".to_string(),
        credentials: Credentials {
            mail: "someEmail3@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::with_dns(&dns);
    assert!(!has_existing_entry(&account, &post_obj.credentials.mail));

    // When
    let req = http_client.post("/create").header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(522, "InvalidNickname"));
    assert!(!has_existing_entry(&account, &post_obj.credentials.mail));
}

#[test]
fn email_in_use() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    // Given
    let http_client = create_http_client(dns.clone());
    let post_obj = CreateMember {
        nickname: "someNickname4".to_string(),
        credentials: Credentials {
            mail: "someEmail4@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let post_obj_used = CreateMember {
        nickname: "someOtherNickname4".to_string(),
        credentials: Credentials {
            mail: "someEmail4@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body_used = serde_json::to_string(&post_obj_used).unwrap();
    let account = Account::with_dns(&dns);
    assert!(!has_existing_entry(&account, &post_obj.credentials.mail));
    // Create an account for this email
    let req = http_client.post("/create").header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(has_existing_entry(&account, &post_obj.credentials.mail));

    // When
    let req_used = http_client.post("/create").header(ContentType::JSON).body(json_body_used.as_str());
    let response_used = req_used.dispatch();

    // Then
    assert_eq!(response_used.status(), Status::new(525, "MailIsInUse"));
}

#[test]
fn email_malformed() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    // Given
    let http_client = create_http_client(dns.clone());
    let post_obj = CreateMember {
        nickname: "someNickname5".to_string(),
        credentials: Credentials {
            mail: "someMalformedEmail5".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::with_dns(&dns);
    assert!(!has_existing_entry(&account, &post_obj.credentials.mail));

    // When
    let req = http_client.post("/create").header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(521, "InvalidMail"));
    assert!(!has_existing_entry(&account, &post_obj.credentials.mail));
}

#[test]
fn password_too_short() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    // Given
    let http_client = create_http_client(dns.clone());
    let post_obj = CreateMember {
        nickname: "someNickname6".to_string(),
        credentials: Credentials {
            mail: "someEmail6@someDomain.test".to_string(),
            password: "tooShort".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::with_dns(&dns);
    assert!(!has_existing_entry(&account, &post_obj.credentials.mail));

    // When
    let req = http_client.post("/create").header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(524, "PasswordTooShort"));
    assert!(!has_existing_entry(&account, &post_obj.credentials.mail));
}

#[test]
fn password_pwned() {
    let container = TestContainer::new(false);
    let (dns, _node) = container.run();

    // Given
    let http_client = create_http_client(dns.clone());
    let post_obj = CreateMember {
        nickname: "someNickname7".to_string(),
        credentials: Credentials {
            mail: "someEmail7@someDomain.test".to_string(),
            password: "correcthorsebatterystaple".to_string(),
        },
    };
    let json_body = serde_json::to_string(&post_obj).unwrap();
    let account = Account::with_dns(&dns);
    assert!(!has_existing_entry(&account, &post_obj.credentials.mail));

    // When
    let req = http_client.post("/create").header(ContentType::JSON).body(json_body.as_str());
    let response = req.dispatch();

    // Then
    assert_eq!(response.status(), Status::new(523, "PwnedPassword"));
    assert!(!has_existing_entry(&account, &post_obj.credentials.mail));
}

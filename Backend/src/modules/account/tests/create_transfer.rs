use rocket::local::Client;
use rocket::http::ContentType;
use crate::modules::account::dto::{CreateMember, Credentials};
use crate::modules::account::Account;
use mysql_connection::tools::Exists;

#[test]
fn create_account_nick_valid_email_valid_password_valid() {
    // Given
    let rocket = rocket::ignite().mount("/", routes![create]);
    let http_client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "someNickName".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let account = Account::default();
    assert!(!account.db_main.exists("SELECT * FROM account_member WHERE mail='someEmail@someDomain.test'"));

    // When
    let req = http_client.post("/create").header(ContentType::JSON).body(post_obj);
    let response = req.dispatch();

    // Then
    assert!(account.db_main.exists("SELECT * FROM account_member WHERE mail='someEmail@someDomain.test'"));
    assert_eq!(response.status(), Status::Ok);

    // Clean up
    account.db_main.execute("DELETE FROM account_member WHERE mail='someEmail@someDomain.test'");
}

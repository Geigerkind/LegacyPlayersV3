use rocket::local::Client;
use rocket::http::ContentType;
use crate::modules::account::dto::{CreateMember, Credentials};
use crate::modules::account::Account;
use mysql_connection::tools::Exists;

#[test]
fn create_account_nick_valid_() {
    // Given
    let rocket = rocket::ignite();
    let client = Client::new(rocket).expect("valid rocket instance");
    let post_obj = CreateMember {
        nickname: "someNickName".to_string(),
        credentials: Credentials {
            mail: "someEmail@someDomain.test".to_string(),
            password: "someExtremelySecurePassword".to_string(),
        },
    };
    let dbClient = Account::default().db_main;
    assert!(!dbClient.exists(email));

    // When
    let req = client.post("/create").header(ContentType::JSON).body(post_obj);
    let response = req.dispatch();

    // Then
    assert!(dbClient.exists(email));
    assert_eq!(response.status(), Status::Ok);

    // Clean up
    account.db_main.execute("DELETE FROM account_member WHERE mail='someEmail@someDomain.test'");
}

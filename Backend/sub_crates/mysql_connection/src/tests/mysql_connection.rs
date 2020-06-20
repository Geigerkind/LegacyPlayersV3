use material::MySQLConnection;

#[test]
#[should_panic]
fn fails_with_unknown_database() {
    let _ = MySQLConnection::new("MyDbName");
}

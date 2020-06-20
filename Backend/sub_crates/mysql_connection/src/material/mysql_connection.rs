use material::DbConnection;

#[derive(Debug)]
pub struct MySQLConnection {
    pub con: mysql::Pool,
}

impl DbConnection for MySQLConnection {
    fn new(dns: &str) -> Self {
        MySQLConnection {
            con: mysql::Pool::new(dns).unwrap(),
        }
    }
}

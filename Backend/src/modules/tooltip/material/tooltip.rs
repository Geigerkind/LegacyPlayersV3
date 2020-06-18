use mysql_connection::material::MySQLConnection;
use std::env;

#[derive(Debug)]
pub struct Tooltip {
    pub db_main: MySQLConnection,
}

impl Default for Tooltip {
    fn default() -> Self {
        let dns = env::var("MYSQL_DNS").unwrap();
        Self::with_dns(&dns)
    }
}

impl Tooltip {
    pub fn with_dns(dns: &str) -> Self {
        Tooltip { db_main: MySQLConnection::new_with_dns(dns) }
    }

    pub fn init(self) -> Self {
        self
    }
}

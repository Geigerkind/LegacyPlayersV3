use std::collections::HashMap;
use std::sync::RwLock;

use language::material::Dictionary;
use mysql_connection::material::MySQLConnection;
use mysql_connection::tools::{Execute, Select};
use str_util::sha3;

use crate::modules::account::language::init::Init;
use crate::modules::account::material::{APIToken, Member};

#[derive(Debug)]
pub struct Account {
  pub db_main: MySQLConnection,
  pub dictionary: Dictionary,
  pub member: RwLock<HashMap<u32, Member>>,
  pub api_token_to_member_id: RwLock<HashMap<String, u32>>,
  pub api_tokens: RwLock<HashMap<u32, Vec<APIToken>>>,
  pub requires_mail_confirmation: RwLock<HashMap<String, u32>>,
}

// Important: Always lock resources bottom to to, in order to prevent running into a deadlock
// Also: Write locks may not be acquired within a query
impl Default for Account {
  fn default() -> Self
  {
    let dictionary = Dictionary::default();
    Dictionary::init(&dictionary);
    Account {
      db_main: MySQLConnection::new("main"),
      dictionary,
      member: RwLock::new(HashMap::new()),
      api_tokens: RwLock::new(HashMap::new()),
      api_token_to_member_id: RwLock::new(HashMap::new()),
      requires_mail_confirmation: RwLock::new(HashMap::new()),
    }
  }
}

impl Account {
  pub fn init(&self)
  {
    let mut requires_mail_confirmation = self.requires_mail_confirmation.write().unwrap();
    let mut api_token_to_member_id = self.api_token_to_member_id.write().unwrap();
    let mut api_token = self.api_tokens.write().unwrap();
    let mut member = self.member.write().unwrap();

    // Cleaning first
    self.clean_tokens();

    // We are a little wasteful here because we do not insert it directly but rather create a vector first and then copy it over
    for entry in self.db_main.select("SELECT id, nickname, mail, password, salt, mail_confirmed, forgot_password, delete_account, new_mail FROM member", &|mut row| {
      Member {
        id: row.take(0).unwrap(),
        nickname: row.take(1).unwrap(),
        mail: row.take(2).unwrap(),
        password: row.take(3).unwrap(),
        salt: row.take(4).unwrap(),
        mail_confirmed: row.take(5).unwrap(),
        forgot_password: row.take(6).unwrap(),
        delete_account: row.take(7).unwrap(),
        new_mail: row.take(8).unwrap(),
      }
    }) {
      // Prepping api_token map
      api_token.insert(entry.id, vec![]);

      // Init remaining confirmation mails
      if !entry.mail_confirmed {
        requires_mail_confirmation.insert(sha3::hash(&[&entry.id.to_string(), "mail", &entry.salt]), entry.id);
      }
      // Init remaining forgot password mails
      if entry.forgot_password {
        requires_mail_confirmation.insert(sha3::hash(&[&entry.id.to_string(), "forgot", &entry.salt]), entry.id);
      }
      // Init remaining delete mails
      if entry.delete_account {
        requires_mail_confirmation.insert(sha3::hash(&[&entry.id.to_string(), "delete", &entry.salt]), entry.id);
      }
      // Init remaining change mail request mails
      if !entry.new_mail.is_empty() {
        requires_mail_confirmation.insert(sha3::hash(&[&entry.id.to_string(), "new_mail", &entry.salt]), entry.id);
      }

      member.insert(entry.id, entry);
    }

    for entry in self.db_main.select("SELECT id, member_id, token, purpose, exp_date FROM api_token", &|mut row| {
      APIToken {
        id: row.take(0).unwrap(),
        member_id: row.take(1).unwrap(),
        token: row.take(2).unwrap(),
        purpose: row.take(3).unwrap(),
        exp_date: row.take(4).unwrap(),
      }
    }) {
      api_token_to_member_id.insert(entry.token.clone(), entry.member_id);
      api_token.get_mut(&entry.member_id).unwrap().push(entry);
    }
  }

  fn clean_tokens(&self) {
    self.db_main.execute("DELETE FROM api_token WHERE exp_date < UNIX_TIMESTAMP()");
  }
}
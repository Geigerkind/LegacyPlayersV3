use language::domain_value::Language;
use language::tools::Get;
use str_util::sha3;

use crate::modules::account::dto::Failure;
use crate::modules::account::material::{Account, APIToken};
use crate::modules::account::tools::Token;

pub trait Login {
  fn login(&self, mail: &str, password: &str) -> Result<APIToken, Failure>;
  fn validate_credentials(&self, mail: &str, password: &str) -> Result<u32, Failure>;
}

impl Login for Account {
  fn login(&self, mail: &str, password: &str) -> Result<APIToken, Failure> {
    self.validate_credentials(mail, password)
      .and_then(|member_id| self.create_token(
        &self.dictionary.get("general.login", Language::English),
        member_id,
        time_util::get_ts_from_now_in_secs(7),
      ))
  }

  fn validate_credentials(&self, mail: &str, password: &str) -> Result<u32, Failure> {
    let lower_mail = mail.to_lowercase();
    for entry in self.member.read().unwrap().values() {
      if entry.mail != lower_mail { continue; }
      if entry.password != sha3::hash(&[&password, &entry.salt]) { break; } // Password is wrong
      return Ok(entry.id);
    }
    Err(Failure::InvalidCredentials)
  }
}
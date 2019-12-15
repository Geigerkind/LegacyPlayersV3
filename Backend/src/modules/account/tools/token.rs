use mysql_connection::tools::{Execute, Select};
use str_util::{random, sha3};
use time_util;

use crate::modules::account::dto::Failure;
use crate::modules::account::material::{Account, APIToken};

pub trait Token {
  fn get_all_token(&self, member_id: u32) -> Vec<APIToken>;
  fn validate_token(&self, api_token: &str) -> Option<u32>;
  fn clear_tokens(&self, member_id: u32) -> Result<(), Failure>;
  fn create_token(&self, purpose: &str, member_id: u32, exp_date: u64) -> Result<APIToken, Failure>;
  fn delete_token(&self, token_id: u32, member_id: u32) -> Result<(), Failure>;
  fn prolong_token(&self, token_id: u32, member_id: u32, days: u32) -> Result<APIToken, Failure>;
}

impl Token for Account {
  fn get_all_token(&self, member_id: u32) -> Vec<APIToken> {
    let api_tokens = self.api_tokens.read().unwrap();
    match api_tokens.get(&member_id) {
      Some(token_vec) => token_vec.to_vec(),
      None => Vec::new()
    }
  }

  fn validate_token(&self, api_token: &str) -> Option<u32> {
    // Check if token exists and if its still valid!
    let mut token_id = None;
    let member_id;
    {
      let api_token_to_member_id = self.api_token_to_member_id.read().unwrap();
      let api_tokens = self.api_tokens.read().unwrap();
      let result = api_token_to_member_id.get(api_token);
      if result.is_none() {
        return None;
      }
      member_id = *result.unwrap();
      let token_vec = api_tokens.get(&member_id).unwrap();
      for entry in token_vec {
        if entry.token == api_token {
          if entry.exp_date >= time_util::now() {
            return Some(member_id);
          }
          token_id = Some(entry.id);
          break;
        }
      }
    }

    // Otherwise delete token and return none!
    if token_id.is_some() {
      let _ = self.delete_token(token_id.unwrap(), member_id);
    }
    None
  }

  fn clear_tokens(&self, member_id: u32) -> Result<(), Failure> {
    let mut api_token_to_member_id = self.api_token_to_member_id.write().unwrap();
    let mut api_token = self.api_tokens.write().unwrap();

    if !self.db_main.execute_wparams("DELETE FROM api_token WHERE member_id=:member_id", params!(
      "member_id" => member_id
    )) {
      return Err(Failure::Unknown);
    }

    for api_token in api_token.get(&member_id).unwrap() {
      api_token_to_member_id.remove(&api_token.token);
    }
    api_token.get_mut(&member_id).unwrap().clear();

    Ok(())
  }

  fn create_token(&self, purpose: &str, member_id: u32, exp_date: u64) -> Result<APIToken, Failure> {
    // Tokens may be valid for a maximum time of a year
    let now = time_util::now();
    if exp_date < now {
      return Err(Failure::DateInThePast);
    }

    if exp_date - now >= 365 * 24 * 60 * 60 {
      return Err(Failure::TooManyDays);
    }

    let purpose_len = purpose.len();
    if purpose_len <= 1 || purpose_len > 24 {
      return Err(Failure::TokenPurposeLength);
    }

    let token: String;
    {
      let member = self.member.read().unwrap();
      let member_entry = member.get(&member_id).unwrap();
      let salt: String = random::alphanumeric(16);
      token = sha3::hash(&[&member_entry.mail, &member_entry.password, &salt]);
    }

    // The following part needs to be transactional
    let mut api_token_to_member_id = self.api_token_to_member_id.write().unwrap();
    let mut api_tokens = self.api_tokens.write().unwrap();

    if !self.db_main.execute_wparams(
      "INSERT INTO api_token (member_id, token, purpose, exp_date) VALUES (:member_id, :token, :purpose, :exp_date)",
      params!(
        "member_id" => member_id,
        "token" => token.clone(),
        "purpose" => purpose,
        "exp_date" => exp_date
      ),
    ) {
      return Err(Failure::Unknown);
    }

    match self.db_main.select_wparams_value(
      "SELECT id, member_id, token, purpose, exp_date FROM api_token WHERE member_id=:member_id AND token=:token",
      &|mut row| {
        APIToken {
          id: row.take(0).unwrap(),
          member_id: row.take(1).unwrap(),
          token: row.take(2).unwrap(),
          purpose: row.take(3).unwrap(),
          exp_date: row.take(4).unwrap(),
        }
      },
      params!(
        "member_id" => member_id,
        "token" => token.clone()
      ),
    ) {
      Some(token) => {
        if api_tokens.get(&member_id).is_none() {
          api_tokens.insert(member_id, vec![token.clone()]);
        } else {
          api_tokens.get_mut(&member_id).unwrap().push(token.clone());
        }
        api_token_to_member_id.insert(token.token.clone(), member_id);
        Ok(token)
      }
      None => return Err(Failure::Unknown)
    }
  }

  fn delete_token(&self, token_id: u32, member_id: u32) -> Result<(), Failure> {
    // We lock before in order to be transactional
    let mut api_token_to_member_id = self.api_token_to_member_id.write().unwrap();
    let mut api_tokens = self.api_tokens.write().unwrap();

    if !self.db_main.execute_wparams(
      "DELETE FROM api_token WHERE id=:id AND member_id=:member_id",
      params!(
        "id" => token_id,
        "member_id" => member_id
      ),
    ) {
      return Err(Failure::Unknown);
    }

    match api_tokens.get(&member_id).unwrap().iter().position(|api_token| api_token.id == token_id) {
      Some(token_index) => {
        let token_vec = api_tokens.get_mut(&member_id).unwrap();
        {
          let api_token = token_vec.get(token_index).unwrap();
          api_token_to_member_id.remove(&api_token.token);
        }
        token_vec.remove(token_index);
        Ok(())
      }
      None => Err(Failure::Unknown)
    }
  }

  fn prolong_token(&self, token_id: u32, member_id: u32, days: u32) -> Result<APIToken, Failure> {
    // Tokens may be valid for a maximum time of a year
    if days >= 365 {
      return Err(Failure::TooManyDays);
    }

    // Continue to update the token
    let mut api_tokens = self.api_tokens.write().unwrap();
    let exp_date = time_util::get_ts_from_now_in_secs(days as u64);
    if self.db_main.execute_wparams("UPDATE api_token SET exp_date=:exp_date WHERE id=:id AND member_id=:member_id", params!(
      "exp_date" => exp_date,
      "id" => token_id,
      "member_id" => member_id
    )) {
      let token_vec = api_tokens.get_mut(&member_id).unwrap();
      let token_pos = token_vec.iter().position(|api_token| api_token.id == token_id).unwrap();
      let api_token = token_vec.get_mut(token_pos).unwrap();
      api_token.exp_date = exp_date;
      return Ok(api_token.clone());
    }
    Err(Failure::Unknown)
  }
}
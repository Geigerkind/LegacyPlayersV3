use language::domain_value::Language;
use language::tools::Get;
use mysql_connection::tools::Execute;
use str_util::{sha3, strformat};
use validator::domain_value::PasswordFailure;
use validator::tools::{valid_mail, valid_nickname, valid_password};

use crate::modules::account::domain_value::AccountInformation;
use crate::modules::account::dto::Failure;
use crate::modules::account::material::{Account, APIToken};
use crate::modules::account::tools::{GetAccountInformation, Token};

pub trait Update {
  fn change_name(&self, new_nickname: &str, member_id: u32) -> Result<AccountInformation, Failure>;
  fn change_password(&self, new_password: &str, member_id: u32) -> Result<APIToken, Failure>;
  fn update_password(&self, new_password: &str, member_id: u32) -> Result<(), Failure>;
  fn request_change_mail(&self, new_mail: &str, member_id: u32) -> Result<bool, Failure>;
  fn confirm_change_mail(&self, confirmation_id: &str) -> Result<APIToken, Failure>;
}

impl Update for Account {
  fn change_name(&self, new_nickname: &str, member_id: u32) -> Result<AccountInformation, Failure>
  {
    if !valid_nickname(new_nickname) {
      return Err(Failure::InvalidNickname);
    }

    {
      let mut member = self.member.write().unwrap();
      // Check if the name exists already
      let lower_name = new_nickname.to_lowercase();
      for entry in member.values() {
        if entry.nickname.to_lowercase() == lower_name
          && entry.id != member_id
        {
          return Err(Failure::NicknameIsInUse);
        }
      }

      if self.db_main.execute_wparams("UPDATE member SET nickname=:nickname WHERE id=:id", params!(
      "nickname" => new_nickname.clone(),
      "id" => member_id
    )) {
        let entry = member.get_mut(&member_id).unwrap();
        entry.nickname = new_nickname.to_owned();
      } else {
        return Err(Failure::NicknameIsInUse);
      }
    }

    Ok(self.get(member_id).unwrap())
  }

  fn change_password(&self, new_password: &str, member_id: u32) -> Result<APIToken, Failure>
  {
    match valid_password(new_password) {
      Err(PasswordFailure::TooFewCharacters) => return Err(Failure::PasswordTooShort),
      Err(PasswordFailure::Pwned(num_pwned)) => return Err(Failure::PwnedPassword(num_pwned)),
      Ok(_) => ()
    };

    self.update_password(new_password, member_id)
      .and_then(|()| self.create_token(
        &self.dictionary
          .get("general.login", Language::English), member_id, time_util::get_ts_from_now_in_secs(7)))
  }

  fn update_password(&self, new_password: &str, member_id: u32) -> Result<(), Failure> {
    let mut member = self.member.write().unwrap();

    let hash: String;
    {
      let entry = member.get(&member_id).unwrap();
      hash = sha3::hash(&[new_password, &entry.salt]);
    }

    if self.db_main.execute_wparams("UPDATE member SET password=:password WHERE id=:id", params!(
      "password" => hash.clone(),
      "id" => member_id
    )) {
      return self.clear_tokens(member_id).and_then(|()| {
        let entry = member.get_mut(&member_id).unwrap();
        entry.password = hash;
        Ok(())
      });
    }
    Err(Failure::Unknown)
  }

  fn request_change_mail(&self, new_mail: &str, member_id: u32) -> Result<bool, Failure>
  {
    if !valid_mail(new_mail) {
      return Err(Failure::InvalidMail);
    }

    let mut requires_mail_confirmation = self.requires_mail_confirmation.write().unwrap();
    let mut member = self.member.write().unwrap();

    // Check if the mail exists already
    let lower_mail = new_mail.to_lowercase();
    for entry in member.values() {
      if (entry.mail == lower_mail || entry.new_mail == lower_mail)
        && entry.id != member_id
      {
        return Err(Failure::MailIsInUse);
      }
    }

    let entry = member.get_mut(&member_id).unwrap();
    // If the mail has not been confirmed yet then it can be changed without
    // confirmation, because the user could have had a typo
    if !entry.mail_confirmed {
      entry.mail = lower_mail.to_owned();
      return Ok(true);
    }

    let confirmation_id = sha3::hash(&[&member_id.to_string(), "new_mail", &entry.salt]);
    entry.new_mail = lower_mail.to_owned();
    requires_mail_confirmation.insert(confirmation_id.clone(), member_id);
    let mail_content = strformat::fmt(self.dictionary.get("update.mail.text", Language::English), &[&confirmation_id]);
    if !mail::send(&entry.mail, &entry.nickname,
                   self.dictionary.get("update.mail.subject", Language::English), mail_content) {
      return Err(Failure::MailSend);
    }
    Ok(false)
  }

  fn confirm_change_mail(&self, confirmation_id: &str) -> Result<APIToken, Failure> {
    let requires_mail_confirmation = self.requires_mail_confirmation.read().unwrap();
    match requires_mail_confirmation.get(confirmation_id) {
      Some(member_id) => {
        {
          let mut member = self.member.write().unwrap();
          let member_entry = member.get_mut(member_id).unwrap();
          let lower_mail = member_entry.new_mail.clone();
          if self.clear_tokens(*member_id).is_ok() && self.db_main.execute_wparams("UPDATE member SET mail=:mail WHERE id=:id", params!(
            "mail" => lower_mail.clone(),
            "id" => member_id
          )) {
            member_entry.mail = lower_mail.to_owned();
            member_entry.new_mail = String::new();
          } else {
            return Err(Failure::Unknown);
          }
        }
        self.create_token(&self.dictionary.get("general.login", Language::English),
                          *member_id, time_util::get_ts_from_now_in_secs(7))
      }
      None => Err(Failure::Unknown)
    }
  }
}
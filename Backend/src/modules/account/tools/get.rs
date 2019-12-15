use crate::modules::account::domain_value::AccountInformation;
use crate::modules::account::dto::Failure;
use crate::modules::account::material::Account;

pub trait GetAccountInformation {
  fn get(&self, id: u32) -> Result<AccountInformation, Failure>;
}

impl GetAccountInformation for Account {
  fn get(&self, id: u32) -> Result<AccountInformation, Failure> {
    let member = self.member.read().unwrap();

    // Although this should never happen;
    // In the right order of execution of independent threads changing the account info, it may happen
    let entry_res = member.get(&id);
    if entry_res.is_none() {
      return Err(Failure::Unknown);
    }

    let entry = entry_res.unwrap();
    Ok(AccountInformation {
      id: entry.id,
      mail: entry.mail.clone(),
      nickname: entry.nickname.clone(),
      mail_confirmed: entry.mail_confirmed,
    })
  }
}
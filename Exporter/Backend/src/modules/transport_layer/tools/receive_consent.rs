use crate::modules::TransportLayer;

pub trait ReceiveConsent {
  fn receive_character_consent(&mut self);
  fn receive_guild_consent(&mut self);
}

impl ReceiveConsent for TransportLayer {
  fn receive_character_consent(&mut self) {
    let receiver = self.receiver_character_consent.as_ref().unwrap();
    loop {
      let received = receiver.try_recv();
      if received.is_ok() {
        match received.unwrap() {
          (false, character_id) => self.character_consent.insert(character_id),
          (true, character_id) => self.character_consent.remove(&character_id)
        };
      } else {
        break;
      }
    }
  }

  fn receive_guild_consent(&mut self) {
    let receiver = self.receiver_guild_consent.as_ref().unwrap();
    loop {
      let received = receiver.try_recv();
      if received.is_ok() {
        match received.unwrap() {
          (false, guild_id) => self.guild_consent.insert(guild_id),
          (true, guild_id) => self.guild_consent.remove(&guild_id)
        };
      } else {
        break;
      }
    }
  }
}
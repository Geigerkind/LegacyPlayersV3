use crate::Run;
use crate::modules::TransportLayer;
use std::thread;
use std::time::Duration;
use crate::modules::transport_layer::tools::ReceiveConsent;

impl Run for TransportLayer {
  fn run(&mut self) {
    loop {
      thread::sleep(Duration::new(1,0));

      self.receive_character_consent();
      self.receive_guild_consent();

      let receiver = self.receiver_character.as_ref().unwrap();
      let received = receiver.try_recv();
      if received.is_ok() {
        println!("{}", received.unwrap());
      }
    }
  }
}
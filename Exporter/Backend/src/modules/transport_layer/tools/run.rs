use std::thread;
use std::time::Duration;

use crate::modules::transport_layer::tools::ReceiveConsent;
use crate::modules::TransportLayer;
use crate::Run;
use reqwest::header::{HeaderValue, CONTENT_TYPE};

impl Run for TransportLayer {
  fn run(&mut self) {
    let rate = 30.0; // TODO: Env
    let api_token = "c7719277bbf252d90427afa088cdaa11898ffcfee09ff073869a61aaff88b686fe3da602642d76ccfbc532c6756cc678df7e5368c0aede8fe83290399699b36e"; // TODO: Env
    let url_set_character = "http://localhost/API/armory/character"; // TODO: Env

    let sleep_duration_rate = Duration::new(0, (1000000000.0 as f64 * (1.0 / rate)).ceil() as u32);
    let sleep_duration_wait = Duration::new(1, 0);
    loop {
      thread::sleep(sleep_duration_rate);

      self.receive_character_consent();
      self.receive_guild_consent();

      let receiver = self.receiver_character.as_ref().unwrap();
      let received_res = receiver.try_recv();
      if received_res.is_ok() {
        let received = received_res.unwrap();
        if !self.character_consent.contains(&received.0) {
          println!("{} ({}) has not given consent, skipping!", received.1.character_history.unwrap().character_name, received.1.server_uid);
          continue;
        }

        let response = self.client
          .post(url_set_character)
          .header("X-Authorization", HeaderValue::from_static(api_token))
          .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
          .body(serde_json::to_string(&received.1).unwrap())
          .send();
        println!("Response okay => {:?}", response.is_ok());
      } else {
        thread::sleep(sleep_duration_wait);
      }
    }
  }
}
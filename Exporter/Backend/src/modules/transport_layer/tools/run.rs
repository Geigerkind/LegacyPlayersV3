use std::{thread, env};
use std::time::Duration;

use crate::modules::transport_layer::tools::ReceiveConsent;
use crate::modules::TransportLayer;
use crate::Run;
use reqwest::header::{HeaderValue, CONTENT_TYPE};

impl Run for TransportLayer {
  fn run(&mut self) {
    let rate = env::var("REQUESTS_TO_LP_PER_SECOND").unwrap().parse::<f64>().unwrap();
    let api_token = env::var("LP_API_TOKEN").unwrap();
    let url_set_character = env::var("URL_SET_CHARACTER").unwrap();
    let opt_in_mode = env::var("OPT_IN_MODE").unwrap().parse::<bool>().unwrap();

    // Relay for Server messages
    let server_plugin_msgs = thread::spawn(|| {
      let context = zmq::Context::new();
      let responder = context.socket(zmq::PULL).unwrap();
      assert!(responder.bind("tcp://127.0.0.1:5690").is_ok());

      loop {
        let msg = responder.recv_bytes(0).unwrap();
        println!("Received {:?}", msg);
      }
    });

    let sleep_duration_rate = Duration::new(0, (1000000000.0 as f64 * (1.0 / rate)).ceil() as u32);
    let sleep_duration_wait = Duration::new(1, 0);
    loop {
      thread::sleep(sleep_duration_rate);

      // Receive new consent decisions
      self.receive_character_consent();
      self.receive_guild_consent();

      // Relay Character DTOs
      let receiver = self.receiver_character.as_ref().unwrap();
      let received_res = receiver.try_recv();
      if received_res.is_ok() {
        let received = received_res.unwrap();
        if (opt_in_mode && !self.character_consent.contains(&received.0)) || (!opt_in_mode && self.character_consent.contains(&received.0)) {
          println!("{} ({}) has not given consent, skipping!", received.1.character_history.unwrap().character_name, received.1.server_uid);
          continue;
        }

        let response = self.client
          .post(&url_set_character)
          .header("X-Authorization", HeaderValue::from_str(api_token.as_str()).unwrap())
          .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
          .body(serde_json::to_string(&received.1).unwrap())
          .send();
        println!("Response okay => {:?} for {} ({})", response.is_ok(), received.1.character_history.unwrap().character_name, received.1.server_uid);
      } else {
        thread::sleep(sleep_duration_wait);
      }
    }
    server_plugin_msgs.join();
  }
}
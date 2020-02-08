use std::thread;
use std::time::Duration;

use crate::modules::ArmoryExporter;
use crate::Run;

impl Run for ArmoryExporter {
  fn run(&mut self) {
    let mut count = 0;
    loop {
      thread::sleep(Duration::new(1, 0));
      count += 1;
      self.sender_character.as_ref().unwrap().send(format!("Ping #{}", count).to_owned());
    }
  }
}
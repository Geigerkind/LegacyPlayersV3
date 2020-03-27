use std::thread;
use crate::modules::transport_layer::tools::{message_build};
use crate::modules::TransportLayer;
use crate::Run;
use crate::modules::transport_layer::tools::relay::Relay;
use std::sync::mpsc;

impl Run for TransportLayer {
  fn run(&mut self) {
    let (tx, rx) = mpsc::channel::<(Vec<u32>, String)>();
    thread::spawn(move || message_build(tx));
    self.relay(rx);
  }
}
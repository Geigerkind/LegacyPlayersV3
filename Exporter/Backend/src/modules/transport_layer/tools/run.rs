use crate::modules::TransportLayer;
use crate::modules::transport_layer::tools::relay::Relay;

impl TransportLayer {
  pub fn run(&mut self) {
    self.relay();
  }
}
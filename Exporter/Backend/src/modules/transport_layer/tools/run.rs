use crate::modules::TransportLayer;
use crate::Run;
use crate::modules::transport_layer::tools::relay::Relay;

impl Run for TransportLayer {
  fn run(&mut self) {
    self.relay();
  }
}
use hyper::Client;

#[derive(Debug)]
pub struct TransportLayer {
  client: Client
}

impl Default for TransportLayer {
  fn default() -> Self {
    TransportLayer {
      client: Client::new()
    }
  }
}

impl TransportLayer {
  pub fn init(self) -> Self
  {
    self
  }
}
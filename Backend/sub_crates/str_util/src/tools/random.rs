use std::iter;

use rand::{Rng, thread_rng};
use rand_distr::Alphanumeric;

pub fn alphanumeric(length: usize) -> String
{
  let mut rng = thread_rng();
  iter::repeat(())
    .map(|()| rng.sample(Alphanumeric))
    .take(length)
    .collect::<String>()
}
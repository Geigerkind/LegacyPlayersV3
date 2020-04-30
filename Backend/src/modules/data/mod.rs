pub use self::{domain_value::Stat, material::Data};

#[cfg(test)]
#[cfg(feature = "integration")]
mod tests;

mod domain_value;
mod dto;
mod language;
mod material;

pub mod guard;
pub mod tools;
pub mod transfer;

pub use self::material::Data;
pub use self::domain_value::Stat;

#[cfg(test)]
mod tests;

mod material;
mod domain_value;
mod dto;
mod language;

pub mod tools;
pub mod transfer;
pub mod guard;
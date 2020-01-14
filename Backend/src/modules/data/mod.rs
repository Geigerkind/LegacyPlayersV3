pub use self::material::Data;

#[cfg(test)]
mod tests;

mod tools;
mod material;
mod domain_value;
mod dto;

pub mod transfer;
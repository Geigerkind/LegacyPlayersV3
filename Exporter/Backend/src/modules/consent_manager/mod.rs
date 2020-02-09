pub use self::material::ConsentManager;

#[cfg(test)]
mod tests;

mod tools;
mod material;
mod guard;
mod domain_value;

pub mod transfer;
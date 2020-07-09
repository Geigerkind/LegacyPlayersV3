pub use self::material::ConsentManager;

#[cfg(test)]
mod tests;

mod domain_value;
mod guard;
mod material;
mod tools;

pub mod transfer;

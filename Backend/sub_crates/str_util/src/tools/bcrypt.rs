use bc;

pub fn hash(input: &[&str]) -> String {
  bc::hash(input.concat(), bc::DEFAULT_COST).unwrap()
}

pub fn hash_one(input: &str) -> String {
  bc::hash(input, bc::DEFAULT_COST).unwrap()
}
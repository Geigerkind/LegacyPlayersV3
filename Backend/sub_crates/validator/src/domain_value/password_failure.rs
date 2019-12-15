pub enum PasswordFailure {
  TooFewCharacters,
  Pwned(u64),
}
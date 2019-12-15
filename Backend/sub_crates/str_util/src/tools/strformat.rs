// Horrible implementation, lets hope there is a better solution at some point!
pub fn fmt(mut format: String, arguments: &[&str]) -> String {
  for i in { 0..arguments.len() } {
    format = format.replace(&format!("{{{}}}", i), arguments[i]);
  }
  format
}
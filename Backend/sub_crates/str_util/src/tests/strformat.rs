extern crate proptest;
use self::proptest::prelude::*;
use crate::strformat;

proptest! {
  #[test]
  fn test_arguments(size in 0usize..33usize, permutations in prop::array::uniform32(0usize..32usize), strings in prop::array::uniform32("\\PC*")) {
    let format = (0..size).map(|i| format!("{{{}}}", permutations[i])).collect::<Vec<_>>().join(" ");
    let arguments = strings.iter().map(|item| &item[..]).collect::<Vec<_>>();
    let expected = (0..size).map(|i| &strings[permutations[i]][..]).collect::<Vec<_>>().join(" ");
    assert_eq!(strformat::fmt(format, &arguments), expected);
  }
}

#[test]
fn fmt() {
  let fmt = "{2} {1} {3} {0}".to_string();
  let input = ["Test", "is", "This", "a"];
  let result = "This is a Test";
  assert_eq!(strformat::fmt(fmt, &input), result);
}

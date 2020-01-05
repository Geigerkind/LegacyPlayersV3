#[cfg(test)]
mod tests {
  use crate::modules::data::Data;
  use crate::modules::data::tools::RetrieveSpell;

  #[test]
  fn get_spell() {
    let data = Data::default().init();
    let spell = data.get_spell(1, 1);
    assert!(spell.is_some());
    let unpacked_spell = spell.unwrap();
    assert_eq!(unpacked_spell.id, 1);
    assert_eq!(unpacked_spell.expansion_id, 1);
    let no_spell = data.get_spell(0, 0);
    assert!(no_spell.is_none());
  }
}
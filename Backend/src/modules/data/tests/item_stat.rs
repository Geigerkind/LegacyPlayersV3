#[cfg(test)]
mod tests {
  use crate::modules::data::Data;
  use crate::modules::data::tools::RetrieveItemStat;

  #[test]
  fn get_item_stats() {
    let data = Data::default().init(Some(28));
    let item_stats = data.get_item_stats(1, 39);
    assert!(item_stats.is_some());
    let item_stats_vec = item_stats.unwrap();
    assert!(item_stats_vec.len() > 0);
    assert_eq!(item_stats_vec[0].expansion_id, 1);
    assert_eq!(item_stats_vec[0].item_id, 39);
    let no_item_stats = data.get_item_stats(0, 0);
    assert!(no_item_stats.is_none());
  }
}
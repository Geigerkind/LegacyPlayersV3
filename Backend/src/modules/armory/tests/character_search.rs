use crate::modules::armory::Armory;
use crate::modules::data::Data;
use crate::modules::armory::tools::PerformCharacterSearch;
use crate::modules::armory::dto::CharacterSearchFilter;
use crate::dto::TableFilter;

#[test]
fn character_search() {
  let armory = Armory::default();
  let data = Data::default().init(None);
  let filter1 = CharacterSearchFilter {
    page: 0,
    hero_class: TableFilter { filter: None, sorting: None },
    name: TableFilter { filter: None, sorting: None },
    guild: TableFilter { filter: None, sorting: None },
    server: TableFilter { filter: None, sorting: None },
    last_updated: TableFilter { filter: None, sorting: None }
  };

  let csr1 = armory.get_character_search_result(&data, filter1.clone());
  assert_eq!(csr1.num_items, 0);
  assert_eq!(csr1.result.len(), 0);
}
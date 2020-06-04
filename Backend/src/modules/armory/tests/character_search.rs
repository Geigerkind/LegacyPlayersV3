use crate::{
    dto::TableFilter,
    modules::{
        armory::{dto::CharacterSearchFilter, tools::PerformCharacterSearch, Armory},
        data::Data,
    },
};
use crate::start_test_db;

#[test]
fn character_search() {
    let dns: String;
    start_test_db!(false, dns);

    let armory = Armory::with_dns((dns.clone() + "main").as_str());
    let data = Data::with_dns((dns + "main").as_str()).init(None);
    let filter1 = CharacterSearchFilter {
        page: 0,
        hero_class: TableFilter { filter: None, sorting: None },
        name: TableFilter { filter: None, sorting: None },
        guild: TableFilter { filter: None, sorting: None },
        server: TableFilter { filter: None, sorting: None },
        last_updated: TableFilter { filter: None, sorting: None },
    };

    let csr1 = armory.get_character_search_result(&data, filter1.clone());
    assert_eq!(csr1.num_items, 0);
    assert_eq!(csr1.result.len(), 0);
}

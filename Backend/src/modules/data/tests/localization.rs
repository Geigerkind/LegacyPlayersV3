use crate::modules::data::{tools::RetrieveLocalization, Data};
use crate::tests::TestContainer;

#[test]
fn get_localization() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns((dns + "main").as_str()).init(Some(3));
    let localization = data.get_localization(1, 1);
    assert!(localization.is_some());
    let unwrapped_localization = localization.unwrap();
    assert_eq!(unwrapped_localization.id, 1);
    assert_eq!(unwrapped_localization.language_id, 1);
    let no_localization = data.get_localization(0, 0);
    assert!(no_localization.is_none());
}

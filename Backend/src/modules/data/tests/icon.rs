use crate::modules::data::tools::RetrieveIcon;
use crate::modules::data::Data;

#[test]
fn get_icon() {
    let data = Data::default().init(Some(14));
    let icon = data.get_icon(1);
    assert!(icon.is_some());
    assert_eq!(icon.unwrap().id, 1);
    let no_icon = data.get_icon(0);
    assert!(no_icon.is_none());
}

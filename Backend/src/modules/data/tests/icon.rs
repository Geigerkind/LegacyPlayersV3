use crate::modules::data::{tools::RetrieveIcon, Data};
use crate::tests::TestContainer;

#[test]
fn get_icon() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let data = Data::with_dns(&dns).init(Some(14));
    let icon = data.get_icon(1);
    assert!(icon.is_some());
    assert_eq!(icon.unwrap().id, 1);
    let no_icon = data.get_icon(0);
    assert!(no_icon.is_none());
}

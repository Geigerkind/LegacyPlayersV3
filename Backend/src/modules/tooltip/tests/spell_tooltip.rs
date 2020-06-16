use crate::modules::{
    data::Data,
    tooltip::{tools::RetrieveSpellTooltip, Tooltip},
};
use crate::tests::TestContainer;

#[test]
fn thunderfury_effect() {
    let container = TestContainer::new(true);
    let (dns, _node) = container.run();

    let tooltip = Tooltip::with_dns((dns.clone() + "main").as_str()).init();
    let data = Data::with_dns((dns + "main").as_str()).init(None);

    let result = tooltip.get_spell(&data, 1, 1, 21992);
    assert!(result.is_ok());

    let spell_tooltip = result.unwrap();
    assert_eq!(spell_tooltip.name, "Thunderfury");
    assert_eq!(spell_tooltip.icon, "spell_nature_cyclone");
    assert_eq!(spell_tooltip.subtext, "");
    assert!(spell_tooltip.spell_cost.is_none());
    assert_eq!(spell_tooltip.range, 5);
    assert_eq!(
        spell_tooltip.description,
        "Blasts your enemy with lightning, dealing 300 Nature damage and then jumping to additional nearby enemies.  Each jump reduces that victim's Nature resistance by 25. Affects 5 targets. Your primary target is also consumed by a cyclone, \
         slowing its attack speed by 20% for 12 seconds."
    );
}

use crate::modules::live_data_processor::tools::payload_mapper::aura_application::MapAuraApplication;

#[test]
fn map_aura_application_positive() {
  // Arrange
  let payload = vec![
    1, 0, 0, 0, 0, 0, 0, 0, // Caster
    42, 0, 0, 0, 0, 0, 0, 0, // Target
    23, 0, 0, 0, // SpellId
    3, // StackAmount
    1 // Applied
  ];

  // Act
  let aura_application = payload.to_aura_application();

  // Assert
  assert!(aura_application.is_ok());
  let aura_application = aura_application.unwrap();
  assert_eq!(aura_application.caster, 1);
  assert_eq!(aura_application.target, 42);
  assert_eq!(aura_application.spell_id, 23);
  assert_eq!(aura_application.stack_amount, 3);
  assert_eq!(aura_application.applied, true);
}

#[test]
fn map_aura_application_negative() {
  // Arrange
  let payload = vec![1,2,3,4,5];

  // Act
  let aura_application = payload.to_aura_application();

  // Assert
  assert!(aura_application.is_err());
}
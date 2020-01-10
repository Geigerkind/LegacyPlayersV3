#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ItemClass {
  pub id: u8,
  pub item_class: u8,
  pub item_sub_class: u8,
  pub localization_id: u32
}
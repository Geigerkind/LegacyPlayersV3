#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TableFilter<T> {
  pub filter: Option<T>,
  pub sorting: Option<bool>
}
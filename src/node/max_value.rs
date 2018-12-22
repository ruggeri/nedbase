#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MaxValue {
  Infinity,
  DefiniteValue(String),
}

impl MaxValue {
  pub fn is_ge_to(&self, value: &str) -> bool {
    match self {
      MaxValue::Infinity => true,
      MaxValue::DefiniteValue(max_value) => value <= max_value.as_str(),
    }
  }
}

// Used in the B-Link tree InteriorNodes to know the max value that can
// be stored in the subtree (specifically, the rightmost branch). That
// can tell us when to move to the right, versus descending.
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

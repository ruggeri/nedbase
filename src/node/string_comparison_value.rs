use std::borrow::Borrow;

// Used in the B-Link tree InteriorNodes to know the max value that can
// be stored in the subtree (specifically, the rightmost branch). That
// can tell us when to move to the right, versus descending.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StringComparisonValue<T>
where
  T: Borrow<str>,
{
  NegativeInfinity,
  DefiniteValue(T),
  Infinity,
}

// pub type StringComparisonValue = StringComparisonValueT<String>;
// pub type StringComparisonRef<'a> = StringComparisonValueT<&'a str>;

impl<T> StringComparisonValue<T>
where
  T: Borrow<str>,
{
  pub fn as_ref(&self) -> StringComparisonValue<&str> {
    match self {
      StringComparisonValue::NegativeInfinity => {
        StringComparisonValue::NegativeInfinity
      }

      StringComparisonValue::DefiniteValue(self_value) => {
        StringComparisonValue::DefiniteValue(self_value.borrow())
      }

      StringComparisonValue::Infinity => {
        StringComparisonValue::Infinity
      }
    }
  }

  pub fn is_ge_to(&self, value: &str) -> bool {
    match self {
      StringComparisonValue::NegativeInfinity => false,

      StringComparisonValue::DefiniteValue(self_value) => {
        value <= self_value.borrow()
      }

      StringComparisonValue::Infinity => true,
    }
  }
}

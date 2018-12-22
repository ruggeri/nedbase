use std::borrow::Borrow;

// This enum is used by InteriorNode/LeafNode to tell the user in which
// way they should move to move toward a target key.
pub enum TraversalDirection<T>
where
  T: Borrow<str>,
{
  // Arrived means we are at the LeafNode with the target key.
  Arrived,
  MoveRight { next_node_identifier: T },
  MoveDown { child_node_identifier: T },
}

impl<T: Borrow<str>> TraversalDirection<T> {
  pub fn as_val(&self) -> TraversalDirection<String> {
    match self {
      TraversalDirection::Arrived => TraversalDirection::Arrived,

      TraversalDirection::MoveRight {
        next_node_identifier,
      } => TraversalDirection::MoveRight {
        next_node_identifier: String::from(
          next_node_identifier.borrow(),
        ),
      },

      TraversalDirection::MoveDown {
        child_node_identifier,
      } => TraversalDirection::MoveDown {
        child_node_identifier: String::from(
          child_node_identifier.borrow(),
        ),
      },
    }
  }
}

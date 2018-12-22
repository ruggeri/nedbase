// This enum is used by InteriorNode/LeafNode to tell the user in which
// way they should move to move toward a target key.
pub enum TraversalDirection {
  // Arrived means we are at the LeafNode with the target key.
  Arrived,
  MoveRight { next_node_identifier: String },
  MoveDown { child_node_identifier: String },
}

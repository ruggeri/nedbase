pub enum TraversalDirection {
  Arrived,
  MoveRight { next_node_identifier: String },
  MoveDown { child_node_identifier: String },
}

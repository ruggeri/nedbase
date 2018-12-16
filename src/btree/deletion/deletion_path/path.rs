use super::DeletionAction;

// The DeletionPath consists of all the nodes that get merged (along
// with their merge sibbling), plus eventually the deepest stable
// ancestor (which is mutated by its children's merge). If there is no
// stable ancestor, then the deletion path goes all the way to the root
// identifier.

pub struct DeletionPath {
  pub(super) actions: Vec<DeletionAction>,
}

impl DeletionPath {
  // Pops the most recent action.
  pub fn pop_action(&mut self) -> DeletionAction {
    self
      .actions
      .pop()
      .expect("path was empty: cannot pop last action")
  }

  // Pushes on a new action.
  pub fn push_action(&mut self, action: DeletionAction) {
    self.actions.push(action);
  }
}

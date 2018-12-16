use super::DeletionAction;

// A DeletionPath consists of a series of DeletionActions. Each action
// holds the locks needed to do its job, plus the logic it must perform.

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

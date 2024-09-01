use core::fmt;
use git2::Error as Git2Error;

use crate::git_action::GitActionType;

#[derive(Debug)]
pub enum ActionError {
  GitError(Git2Error),
  ExecutionError(String),
}

impl From<Git2Error> for ActionError {
  fn from(error: Git2Error) -> Self {
    ActionError::GitError(error)
  }
}
impl fmt::Display for GitActionType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      GitActionType::PULL => write!(f, "PULL"),
      GitActionType::PUSH => write!(f, "PUSH"),
    }
  }
}
impl fmt::Display for ActionError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ActionError::GitError(msg) => write!(f, "Git error: {}", msg),
      ActionError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
    }
  }
}

use std::fmt;

use git2::Error as GitError;

use crate::action::ActionEnum;

impl fmt::Display for ActionEnum {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ActionEnum::PULL => write!(f, "PULL"),
      ActionEnum::PUSH => write!(f, "PUSH"),
      ActionEnum::SYNC => write!(f, "SYNC"),
    }
  }
}

#[derive(Debug)]
pub enum ActionError {
  GitError(GitError),
  ExecutionError(String),
}

impl fmt::Display for ActionError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ActionError::GitError(msg) => write!(f, "Git error: {}", msg),
      ActionError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
    }
  }
}

impl std::error::Error for ActionError {}

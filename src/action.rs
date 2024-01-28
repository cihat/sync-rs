use std::fmt;

use crate::project::Project;
use once_cell::sync::Lazy;

pub trait Action {
  fn run(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>>;
}

pub enum ActionType {
  GitSync,
  CustomCommand,
}

impl fmt::Display for ActionType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ActionType::GitSync => write!(f, "Git Sync"),
      ActionType::CustomCommand => write!(f, "Custom Command"),
    }
  }
}

impl ActionType {
  pub fn iter() -> std::slice::Iter<'static, ActionType> {
    static ACTIONS: [ActionType; 2] = [ActionType::GitSync, ActionType::CustomCommand];
    ACTIONS.iter()
  }
}

pub struct ActionFactory;

impl ActionFactory {
  pub fn get_action(action_type: &ActionType) -> Box<dyn Action> {
    match action_type {
      ActionType::GitSync => Box::new(GitCommand {
        git_command_type: GitCommandType::GitSync {
          remote: lazy_remote(),
          branch: lazy_branch(),
        },
      }),
      ActionType::CustomCommand => Box::new(CustomCommand {}),
    }
  }
}

// Use once_cell to store values lazily
static REMOTE: Lazy<String> =
  Lazy::new(|| inquire::Text::new("Enter the remote: ").prompt().unwrap());
static BRANCH: Lazy<String> =
  Lazy::new(|| inquire::Text::new("Enter the branch: ").prompt().unwrap());

fn lazy_remote() -> String {
  REMOTE.clone()
}

fn lazy_branch() -> String {
  BRANCH.clone()
}

//TODO: implement git commands with git2 crate
// trait GitCommandTrait {
//   fn pull(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>>;
//   fn push(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>>;
//   fn commit(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>>;
// }

enum GitCommandType {
  GitSync { remote: String, branch: String },
}

struct GitCommand {
  git_command_type: GitCommandType,
}

impl Action for GitCommand {
  fn run(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>> {
    if !project.is_git {
      return Ok(());
    }

    match &self.git_command_type {
      GitCommandType::GitSync { remote, branch } => {
        println!("{:?} {:?}", remote, branch);
        println!("git sync");
        println!("project name: {:?}\n", project);
      }
    }

    Ok(())
  }
}

struct CustomCommand {}

impl Action for CustomCommand {
  fn run(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>> {
    println!("custom command");
    println!("project name: {:?}\n", project);
    Ok(())
  }
}

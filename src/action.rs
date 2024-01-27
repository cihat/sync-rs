use std::fmt;

use crate::project::Project;

pub trait Action {
  fn run(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>>;
}

pub enum ActionType {
  GitAction,
  CustomCommand,
}

impl fmt::Display for ActionType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ActionType::GitAction => write!(f, "Git Action"),
      ActionType::CustomCommand => write!(f, "Custom Command"),
    }
  }
}

impl ActionType {
  pub fn iter() -> std::slice::Iter<'static, ActionType> {
    static ACTION_TYPES: [ActionType; 2] = [ActionType::GitAction, ActionType::CustomCommand];
    ACTION_TYPES.iter()
  }
}

pub struct ActionFactory;

impl ActionFactory {
  pub fn get_action(action_type: &ActionType) -> Box<dyn Action> {
    match action_type {
      ActionType::GitAction => Box::new(GitCommand {}),
      ActionType::CustomCommand => Box::new(CustomCommand {}),
    }
  }
}

struct GitCommand {}

impl Action for GitCommand {
  fn run(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>> {
    if !project.is_git {
      return Ok(());
    }
    println!("git command");
    println!("project name: {:?}\n", project);
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

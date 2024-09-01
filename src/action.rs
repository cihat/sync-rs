use crate::{error::ActionError, git_action::GitActionType, project::Project};

#[allow(dead_code)]
pub trait ActionTrait {
  fn run(&self) -> Result<(), ActionError>;
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Action {
  pub projects: Vec<Project>,
  pub actions: Vec<String>,
  pub origin: String,
  pub branch: String,
}

#[allow(dead_code)]
impl ActionTrait for Action {
  fn run(&self) -> Result<(), ActionError> {
    for project in &self.projects {
      if !project.is_git {
        println!(
          "{a} project have not a git repository. Skipping...",
          a = project.name
        );
        continue;
      }

      println!("Executing action for {a} project...", a = project.name);

      GitActionType::PULL.action(&project);
      GitActionType::PUSH.action(&project);
      GitActionType::SYNC.action(&project);
    }

    return Ok(());
  }
}

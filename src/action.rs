use crate::{answer::Answer, error::ActionError, git_action::GitActionType};

pub trait ExecutableAction {
  fn run(&self) -> Result<(), ActionError>;
}

impl ExecutableAction for Answer {
  fn run(&self) -> Result<(), ActionError> {
    for project in &self.projects {
      if !project.is_git {
        println!(
          "{} project does not have a git repository. Skipping...",
          project.name
        );
        continue;
      }

      println!("Executing action for {} project...", project.name);

      GitActionType::PULL.action(&project, &self.origin, &self.branch)?;
      GitActionType::PUSH.action(&project, &self.target, &self.branch)?;
    }

    Ok(())
  }
}

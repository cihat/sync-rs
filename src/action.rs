use crate::{error::ActionError, project::Project};

#[derive(Debug)]
#[allow(dead_code)]
pub enum ActionEnum {
  PULL,
  PUSH,
  SYNC,
}

impl ActionEnum {
  fn action(&self, project: &Project) -> Result<(), ActionError> {
    match self {
      ActionEnum::PULL => {
        println!("Project: {}", project.name);
        println!("Executing PULL action...");
        // Implement the actual pull action logic here
        Ok(())
      }
      ActionEnum::PUSH => {
        println!("Project: {}", project.name);
        println!("Executing PUSH action...");
        // Implement the actual push action logic here
        Ok(())
      }
      ActionEnum::SYNC => {
        println!("Project: {}", project.name);
        println!("Executing SYNC action...");
        // Implement the actual sync action logic here
        Ok(())
      }
    }
  }
}

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

      ActionEnum::PULL.action(&project);
      ActionEnum::PUSH.action(&project);
      ActionEnum::SYNC.action(&project);
    }

    return Ok(());
  }
}

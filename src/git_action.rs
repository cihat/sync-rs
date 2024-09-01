use crate::{error::ActionError, project::Project};

#[derive(Debug)]
#[allow(dead_code)]
pub enum GitActionType {
  PULL,
  PUSH,
  SYNC,
}

impl GitActionType {
  pub fn action(&self, project: &Project) -> Result<(), ActionError> {
    match self {
      GitActionType::PULL => {
        println!("Project: {}\nExecuting PULL action...", project.name);
        println!("Implement the actual pull action logic here");

        // todo!("Implement the actual pull action logic here");
        Ok(())
      }
      GitActionType::PUSH => {
        println!("Project: {}\nExecuting PUSH action...", project.name);
        println!("Implement the actual push action logic here");

        // todo!("Implement the actual push action logic here");
        Ok(())
      }
      GitActionType::SYNC => {
        println!("Project: {}\nExecuting SYNC action...", project.name);
        println!("Implement the actual sync action logic here");

        // todo!("Implement the actual sync action logic here");
        Ok(())
      }
    }
  }
}

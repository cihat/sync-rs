use crate::action::{ActionFactory, ActionType};
use crate::project::Project;
use inquire::{formatter::MultiOptionFormatter, MultiSelect};

pub fn select_projects(
  projects: Vec<Project>,
) -> Result<Vec<Project>, inquire::error::InquireError> {
  let formatter: MultiOptionFormatter<'_, Project> = &|a| format!("{} different projects", a.len());

  let ans = MultiSelect::new("Select the projects:", projects)
    .with_formatter(formatter)
    .prompt();

  match ans {
    Ok(selected) => Ok(selected),
    Err(e) => Err(e),
  }
}

pub fn action_run(projects: Vec<Project>) {
  

  let ans = MultiSelect::new(
    "Select the action:",
    ActionType::iter().collect::<Vec<&ActionType>>(),
  )
  .prompt();

  match ans {
    Ok(actions) => {
      projects.iter().for_each(|project| {
        actions.iter().for_each(|action_type| {
          let action = ActionFactory::get_action(&action_type);
          action.run(project).unwrap();
        });
      });
    }
    Err(e) => {
      eprintln!("Error: {:?}", e);
      std::process::exit(1);
    }
  }
}

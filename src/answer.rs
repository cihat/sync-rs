use std::process;

use crate::action::{ActionFactory, ActionType};
use crate::project::Project;
use inquire::{formatter::MultiOptionFormatter, MultiSelect};

pub fn get_workdir() -> Vec<Project> {
  let exclude_folders = vec![".DS_Store".to_string(), "node_modules".to_string()];
  let projects = Project::get_projects(
    "/Users/cihatsalik/github/source".to_string(),
    // workspace_path(),
    exclude_folders,
  );

  let selected_project = select_projects(projects);

  if let Err(e) = selected_project {
    eprintln!("Error: {:?}", e);
    process::exit(1);
  }

  selected_project.unwrap()
}

//TODO: add validation and implement this function for dynamic path
fn workspace_path() -> String {
  let ans = inquire::Text::new("Enter the workspace path:").prompt();

  // if ans.unwrap().is_empty() {
  //   return "/Users/cihatsalik/github/source";
  // }

  match ans {
    Ok(path) => path,
    Err(e) => {
      eprintln!("Error: {:?}", e);
      std::process::exit(1);
    }
  }
}

fn select_projects(projects: Vec<Project>) -> Result<Vec<Project>, inquire::error::InquireError> {
  let formatter: MultiOptionFormatter<'_, Project> = &|a| format!("{} different projects", a.len());

  let ans = MultiSelect::new("Select the projects:", projects)
    .with_formatter(formatter)
    .prompt();

  Ok(ans.unwrap())
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
          let action = ActionFactory::get_action(action_type);
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

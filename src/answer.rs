use std::process;

use crate::project::Project;
use inquire::{formatter::MultiOptionFormatter, MultiSelect};

#[derive(Debug)]
pub struct Answer {
  pub projects: Vec<Project>,
  pub actions: Vec<String>,
  pub origin: String,
  pub branch: String,
}

impl Answer {
  pub fn build() -> Self {
    let projects = Project::get_projects(
      "/Users/cihatsalik/github/source".to_string(),
      // workspace_path(),
      vec![".DS_Store".to_string(), "node_modules".to_string()],
    );
    let selected_project: Result<Vec<Project>, inquire::InquireError> = select_projects(projects);
    let (origin, branch) = get_origin_and_branch();
    let actions = select_actions();

    if let Err(e) = selected_project {
      eprintln!("Error: {:?}", e);
      process::exit(1);
    }

    Self {
      projects: selected_project.unwrap(),
      actions: actions.unwrap(),
      origin,
      branch,
    }
  }
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

fn select_actions() -> Result<Vec<String>, inquire::error::InquireError> {
  let formatter: MultiOptionFormatter<'_, String> = &|a| format!("{} different actions", a.len());

  let ans = MultiSelect::new(
    "Select the actions:",
    vec!["Git Sync".to_string(), "Custom Command".to_string()],
  )
  .with_formatter(formatter)
  .prompt();

  Ok(ans.unwrap())
}

fn get_origin_and_branch() -> (String, String) {
  let remote = inquire::Text::new("Enter the remote: ").prompt().unwrap();
  let branch = inquire::Text::new("Enter the branch: ").prompt().unwrap();

  (remote, branch)
}

use std::process;

mod action;
mod answer;
mod project;

fn main() {
  let exclude_folders = vec![".DS_Store".to_string(), "node_modules".to_string()];
  let projects = project::Project::get_projects(
    "/Users/cihatsalik/github/source".to_string(),
    exclude_folders,
  );

  let selected_project = answer::select_projects(projects);

  if let Err(e) = selected_project {
    eprintln!("Error: {:?}", e);
    process::exit(1);
  }

  answer::action_run(selected_project.unwrap());
}

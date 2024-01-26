mod inquire;
mod project;

fn main() {
  let exclude_folders = vec![".DS_Store".to_string(), "node_modules".to_string()];
  let projects = project::Project::get_projects(
    "/Users/cihatsalik/github/source".to_string(),
    exclude_folders,
  );

  let selected_project = inquire::multiple_select(projects);

  if let Ok(selected_project) = selected_project {
    println!("{:#?}", selected_project);
  } else {
    println!("Error: {:?}", selected_project);
  }
}

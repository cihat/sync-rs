use std::fs;

#[derive(Debug)]
pub struct Project {
  pub path: String,
  pub name: String,
  pub is_git: bool,
}

impl Project {
  fn new(path: String, name: String, is_git: bool) -> Project {
    Project { path, name, is_git }
  }

  pub fn get_projects(workspace_path: String, exclude_folders: Vec<String>) -> Vec<Project> {
    let mut projects: Vec<Project> = Vec::new();
    let paths = fs::read_dir(workspace_path).unwrap();

    for path in paths {
      let path = path.unwrap().path();
      let name = path.file_name().unwrap().to_str().unwrap().to_string();
      let is_git = path.join(".git").exists();

      if exclude_folders.contains(&name) {
        continue;
      }

      projects.push(Project::new(
        path.to_str().unwrap().to_string(),
        name,
        is_git,
      ));
    }
    projects
  }
}

use crate::project::Project;
use inquire::{formatter::MultiOptionFormatter, MultiSelect};

pub fn multiple_select(
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

impl std::fmt::Display for Project {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

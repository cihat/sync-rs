//TODO: implement action type

// use crate::project::Project;

// #[derive(Debug)]
// pub enum ActionType {
//   GitSync { origin: String, branch: String },
//   CustomCommand { command: String },
// }

// impl ActionType {
//   fn execute(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>> {
//     match self {
//       ActionType::GitSync { origin, branch } => {
//         println!("{:?} {:?}", origin, branch);
//         println!("git sync");
//         println!("project name: {:?}\n", project);
//       }
//       ActionType::CustomCommand { command } => {
//         println!("{:?}", command);
//         println!("custom command");
//         println!("project name: {:?}\n", project);
//       }
//     }

//     Ok(())
//   }
// }

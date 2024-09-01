mod action;
mod answer;
mod project;
mod error;

use action::{Action, ActionTrait};
use answer::Answer;
use std::process;

fn main() {
  let answers = Answer::build();
  println!("{:#?}", answers);

  let action = Action {
    projects: answers.projects,
    actions: answers.actions,
    origin: answers.origin,
    branch: answers.branch,
  };

  match action.run() {
    Ok(_) => {
      println!("Action completed successfully");
      process::exit(0);
    }

    Err(e) => {
      eprintln!("{}", e);
      process::exit(1);
    }
  }
}

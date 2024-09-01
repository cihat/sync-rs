mod action;
mod answer;
mod project;
mod error;

use answer::Answer;
use std::process;

fn main() {
  let answers = Answer::build();

  let Answer {
    projects,
    actions,
    origin,
    branch,
  } = answers;

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

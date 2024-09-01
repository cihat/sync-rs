mod action;
mod answer;
mod error;
mod git_action;
mod project;

use action::ExecutableAction;
use answer::Answer;
use std::process;

fn main() {
  let answers = Answer::build();
  println!("{:#?}", answers);

  match answers.run() {
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

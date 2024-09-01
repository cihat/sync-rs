mod action;
mod answer;
mod project;

use answer::Answer;

fn main() {
  let answers = Answer::build();

  let Answer {
    projects,
    actions,
    origin,
    branch,
  } = answers;
}

mod action;
mod answer;
mod project;
use answer::action_run;

fn main() {
  let answers = answer::get_workdir();

  action_run(answers);
}

mod action;
mod answer;
mod project;

fn main() {
  let answers = answer::Answer::build();

  println!("{:#?}", answers);
}

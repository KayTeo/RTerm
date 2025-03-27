use std::io;

pub fn readInput() {
  println!("Enter LLM query:");
  let mut input = String::new();
  io::stdin().read_line(&input).expect("Failed  to readline");

  println!("You entered {}", input);
}
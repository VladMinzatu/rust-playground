fn main() {
  println!("This is the all important main function");
  println!("{}", expression());
}

fn mind_the_naming_convention() {
  println!("snake_case!");
  println!("also for vairables, btw");
}

fn expression() -> i32 {
  let a = {
    let b = 0;
    b + 1
  };
  a // no semicolon - that would turn this into a statement
}
fn main() {
  if_else();
}

fn if_else() {
  let x = 1;

  if x < 0 {
    println!("x is negative");
  } else if x > 0 {
    println!("x is positive");
  } else {
    println!("x is zero");
  }
}
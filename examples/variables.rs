fn main() {
  shadowing();
}

fn mutable_variables() {
  let mut x = 5;
  println!("x is {x}");
  x = 6;
  println!("x is now {x}");
}

fn shadowing() {
  let x = 5;
  let x = x + 1;
  {
    let x = x * 2;
    println!("x is {x}");
  }
  println!("x is {x}");
}
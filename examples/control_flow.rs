fn main() {
  for_loop_with_range();
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

fn if_is_an_expression() {
  let x = if true { 1 } else { 0 };
  println!("x is {x}");
}

fn loop_forever() {
  loop {
    println!("again");
  }
}

fn loop_with_break() {
  let mut x = 0;
  let result = loop {
    x += 1;
    if x == 10 {
      break x * 2;
    }
  };
  println!("result is {result}");
}

fn while_loop() {
  let mut x = 0;
  while x < 10 {
    println!("x is {x}");
    x += 1;
  }
}

fn for_loop() {
  let a = [10, 20, 30, 40, 50];
  for element in a {
    println!("the value is: {element}");
  }
}

fn for_loop_with_range() {
  for number in (1..4) {
    println!("the value is: {number}");
  }
}
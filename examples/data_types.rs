fn main() {
  tuples();
}

fn ints() {
  let x: i32 = 5_000;
  let y: i32 = 0o64;
  println!("x + y = {}", x + y);
}

fn bools() {
  let t: bool = true;
  let f: bool = false;
  println!("t is {t} and f is {f}");
}

fn chars() {
  let c: char = 'z';
  println!("c is {c}");
}

fn tuples() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("x is {x}, y is {y}, z is {z}");
  println!("tup is {tup:?}");
  println!("tup.0 is {}", tup.0);
  println!("tup.1 is {}", tup.1);
  println!("tup.2 is {}", tup.2);

  let mut tup2: (i32, i32) = (5, 6);
  tup2.0 = 10;
  println!("tup2 is {tup2:?}");
}
fn main() {
  bools();
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
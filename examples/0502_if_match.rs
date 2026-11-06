fn main() {
    let x = 5;
    let y = if x < 10 {
        println!("x is less than 10");
        x * 2
    } else if x == 10 {
        println!("x is equal to 10");
        x
    } else {
        println!("x is greater than 10");
        x / 2
    };
    println!("y = {}", y);

    match y {
        10 => println!("x was less than 10"),
        5 => println!("x was equal to 10"),
        2 => println!("x was greater than 10"),
        _ => println!("unexpected value"),
    }
}
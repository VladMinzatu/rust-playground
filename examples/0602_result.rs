fn main() {
    match get_result(15) {
        Ok(value) => println!("Result is Ok({})", value),
        Err(err) => eprintln!("Error: {err}"),
    }
}

fn get_result(x: i32) -> Result<i32, String> {
    if x < 10 {
        Ok(x * 2)
    } else {
        Err("x is greater than or equal to 10".to_string())
    }
}
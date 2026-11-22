fn main() {
    match propagation_example() {
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

fn propagation_example() -> Result<i32, String> {
    let value = get_result(5)?; // This will return Ok(10)
    println!("Value is: {}", value);
    let another_value = get_result(value)?; // This will return Err since 10 >= 10
    println!("Another Value is: {}", another_value);
    Ok(another_value)
}
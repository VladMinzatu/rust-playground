fn main() {
    match propagation_example() {
        Ok(value) => println!("Result is Ok({})", value),
        Err(err) => eprintln!("Error: {err}"),
    }

    let result = get_result(5);
    println!("Result is Ok: {}", result.is_ok());
    println!("Result is Err: {}", result.is_err());

    result.ok().map(|v| println!("Value is: {}", v)); // ok returns Option<T> and map is called on it to print the value if it exists

    let result2 = get_result(15);
    println!("Unwrapping with fallback: {}", result2.unwrap_or(-1));
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
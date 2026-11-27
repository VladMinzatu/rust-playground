enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500
}

impl HttpStatus {
    fn message(&self) -> &str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::InternalServerError => "Internal Server Error",
        }
    }
}

fn main() {
    let status = HttpStatus::Ok;
    match status {
        HttpStatus::Ok => println!("Status: 200 OK"),
        HttpStatus::NotFound => println!("Status: 404 Not Found"),
        HttpStatus::InternalServerError => println!("Status: 500 Internal Server Error"), 
    }

    assert_eq!(size_of::<HttpStatus>(), 2); // smallest integer type that can hold the values
    assert_eq!(HttpStatus::Ok as i32, 200); // can cast enums to integers

    println!("Status OK message: {}", status.message());
}
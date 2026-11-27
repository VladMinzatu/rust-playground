enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500
}

fn main() {
    let status = HttpStatus::Ok;
    match status {
        HttpStatus::Ok => println!("Status: 200 OK"),
        HttpStatus::NotFound => println!("Status: 404 Not Found"),
        HttpStatus::InternalServerError => println!("Status: 500 Internal Server Error"), 
    }
}
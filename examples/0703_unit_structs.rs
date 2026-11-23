struct FatalError; // occupies no memory space, but can be used as a type 
fn main() {
    let err = FatalError; // it's a type with one value, which is itself
    match err {
        FatalError => println!("A fatal error was encountered."),
    }
}
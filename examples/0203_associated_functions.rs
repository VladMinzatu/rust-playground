// Functions declared in an `impl` block without a `self` parameter are
// associated functions, not methods - called with `Type::function(...)`
// rather than `instance.method(...)`. They're commonly used as constructors.

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // `Self` refers to the type the impl block is for, i.e. `Rectangle`.
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    calling_an_associated_function_as_a_constructor();
    multiple_constructors_on_the_same_type();
}

fn calling_an_associated_function_as_a_constructor() {
    let rect = Rectangle::new(30, 50); // no instance needed to call it
    println!("area: {}", rect.area());
}

fn multiple_constructors_on_the_same_type() {
    let sq = Rectangle::square(20);
    println!("square area: {}", sq.area());
}

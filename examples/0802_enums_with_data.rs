enum Shape { // An enum with data with named fields
    Circle{radius: f64},
    Rectangle{width: f64, height: f64},
}

enum Shape2 { // An enum with tuple variants with unnamed fields
    Circle(f64),
    Rectangle(f64, f64),
}

fn main() {
    let circle = Shape::Circle{radius: 5.0};
    let rectangle = Shape::Rectangle{width: 10.0, height: 20.0};

    match circle {
        Shape::Circle{radius} => println!("Circle with radius: {}", radius),
        Shape::Rectangle{width, height} => println!("Rectangle with width: {} and height: {}", width, height),
    }

    match rectangle {
        Shape::Circle{radius} => println!("Circle with radius: {}", radius),
        Shape::Rectangle{width, height} => println!("Rectangle with width: {} and height: {}", width, height),
    }

    let circle2 = Shape2::Circle(5.0);
    let rectangle2 = Shape2::Rectangle(10.0, 20.0);

    match circle2 {
        Shape2::Circle(radius) => println!("Circle with radius: {}", radius),
        Shape2::Rectangle(width, height) => println!("Rectangle with width: {} and height: {}", width, height),
    }

    match rectangle2 {
        Shape2::Circle(radius) => println!("Circle with radius: {}", radius),
        Shape2::Rectangle(width, height) => println!("Rectangle with width: {} and height: {}", width, height),
    }
}
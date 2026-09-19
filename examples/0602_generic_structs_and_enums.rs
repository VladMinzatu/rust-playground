// Structs and enums can be generic over one or more type parameters too.
// `Option<T>` and `Result<T, E>`, used throughout the earlier examples, are
// exactly this - ordinary enums defined with generics in the standard
// library, nothing special about them beyond that.

struct Point<T> {
    x: T,
    y: T,
}

// A method block for a generic struct repeats the type parameter right
// after `impl` so it's in scope for the method signatures below.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// A method can be constrained more tightly than the struct itself - this
// one only exists on `Point<f64>`, not on `Point<T>` in general.
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Two independent type parameters let the fields differ in type.
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    a_struct_generic_over_one_type();
    a_method_available_only_for_one_concrete_type();
    a_struct_generic_over_two_types();
}

fn a_struct_generic_over_one_type() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("integer_point.x: {}", integer_point.x());
    println!("float_point.x: {}", float_point.x());
}

fn a_method_available_only_for_one_concrete_type() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("distance from origin: {}", p.distance_from_origin());
    // Point { x: 3, y: 4 }.distance_from_origin(); // error[E0599]: no method named `distance_from_origin` found for `Point<i32>`
}

fn a_struct_generic_over_two_types() {
    let mixed = MixedPoint { x: 5, y: 4.0 };
    println!("mixed: ({}, {})", mixed.x, mixed.y);
}

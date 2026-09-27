// A generic function is written once and works over any type that
// satisfies its bounds. The compiler monomorphizes it - generating a
// separate, fully-typed version for each concrete type actually used - so
// there's no runtime cost compared to writing the duplicated versions by
// hand.

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// The two functions above are identical except for the type - exactly what
// a generic function collapses into one. `PartialOrd` is required because
// the body compares elements with `>`, and not every type supports that.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    duplicated_functions_per_type();
    one_generic_function_replaces_both();
}

fn duplicated_functions_per_type() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest_i32(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {}", largest_char(&chars));
}

fn one_generic_function_replaces_both() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {}", largest(&chars));
}

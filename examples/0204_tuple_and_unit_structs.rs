// Tuple structs are named but their fields are positional, not named -
// useful when the field names would just repeat the type name. Unit-like
// structs have no fields at all and are mostly used as markers.

struct Point(i32, i32);
struct Velocity(i32, i32);

struct AlwaysEqual; // a unit-like struct: no fields, one value

fn main() {
    tuple_structs_have_positional_fields();
    distinct_tuple_struct_types_dont_mix();
    unit_like_structs_carry_no_data();
}

fn tuple_structs_have_positional_fields() {
    let origin = Point(0, 0);
    println!("origin: ({}, {})", origin.0, origin.1); // access by index, not name
}

fn distinct_tuple_struct_types_dont_mix() {
    let p = Point(1, 2);
    let v = Velocity(1, 2);
    // let v2: Velocity = p; // error[E0308]: mismatched types - same field types, but Point and Velocity are distinct
    println!("point: ({}, {}), velocity: ({}, {})", p.0, p.1, v.0, v.1);
}

fn unit_like_structs_carry_no_data() {
    let _subject = AlwaysEqual; // no parentheses, no fields - just a value of the type
    println!("unit-like structs exist purely for their type, e.g. as a marker or trait target");
}

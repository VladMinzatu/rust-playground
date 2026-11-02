fn main() {
    let x = 10;
    f(&x);

    let v = vec![1, 2, 3, 4, 5];
    let smallest_value = smallest(&v);
    println!("The smallest value in the vector is: {}", smallest_value);

    let s;
    {
        let w = vec![1,2,3,-1,3];
        s = smallest(&w);
    }
    // assert_eq(s, &-1); // Can't do this because w is dropped at the end of the inner scope, and s is a reference to an element of w, which is no longer valid. This would cause a compile error.
}

fn f<'a>(x: &'a i32) {
    println!("x = {}", x);
}

fn smallest(v : &[i32]) -> &i32 { // takes a reference to a slice of i32 and returns a reference to the smallest element in the slice, an i32
    let mut smallest = &v[0];
    for i in v {
        if i < smallest {
            smallest = i;
        }
    }
    smallest
}
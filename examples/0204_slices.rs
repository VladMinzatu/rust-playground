fn main() {
    let a = [1.0, 2.0, 3.0, 4.0, 5.0];
    let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    print_nums(&a); // works on arrays
    print_nums(&v); // works on vectors

    print_nums(&v[0..2]);  // print the first two elements of v
    print_nums(&a[2..]);   // print elements of a starting with a[2]
}

fn print_nums(nums: &[f64]) {
    for n in nums {
        println!("{n}");
    }
    println!()
}
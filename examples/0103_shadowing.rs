fn main() {
    let x = 5;
    println!("Original x: {}", x);
    {
        let x = x + 1; // Shadowing the outer x
        println!("Shadowed x: {}", x);
    }

    let x = x * 2; // Shadowing again
    println!("Final x: {}", x);
}

fn main() {
    println!("gcd(48, 18) = {}", gcd(48, 18));
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        if b < a {
            let temp= b;
            b = a;
            a = temp;
        }
        b = b % a;
    }
    a
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(18, 48), 6);
    assert_eq!(gcd(7, 3), 1);
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
}
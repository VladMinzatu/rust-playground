fn main() {
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(0xcafedad_u64 as u8, 0xad_u8); // last byte
    assert_eq!(2_u16.pow(4), 16);
    println!("{}", (-4_i32).abs()); // println!("{}", (-4).abs()); won't compile 

    // println!("{}", i32::MAX + 1); -> panics on overflow in debug mode, wraps around in release mode
    assert_eq!(500_i16.wrapping_mul(500), -12144); // behaves like * in release builds

    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);

    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));

    // can convert bools to int
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    let t = (1999_i32, 12_i32, 31_i32, true);
    println!("{:?}", t);
    println!("{}", t.0);
}

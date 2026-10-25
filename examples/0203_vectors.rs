fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    assert_eq!(nums.iter().product::<i32>(), 120);

    nums.push(6);
    assert_eq!(nums.iter().product::<i32>(), 720);

    let v: Vec<i32> = (0..5).collect(); // vec from iterator
    assert_eq!(v, [0, 1, 2, 3, 4]);
}
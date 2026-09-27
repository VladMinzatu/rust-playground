// A mutable reference (`&mut`) allows modifying borrowed data, but the
// borrow checker enforces "aliasing XOR mutation": while a mutable borrow
// is alive, no other reference - shared or mutable - to the same data may
// exist.

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    only_one_mutable_borrow_at_a_time();
    mutable_and_shared_borrows_dont_mix();
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn only_one_mutable_borrow_at_a_time() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // error[E0499]: cannot borrow `s` as mutable more than once at a time
    r1.push('!');
    println!("{r1}");
}

fn mutable_and_shared_borrows_dont_mix() {
    let mut s = String::from("hello");

    let r1 = &s;
    // let r2 = &mut s; // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{r1}");

    // Non-lexical lifetimes: `r1`'s borrow ends at its last use above, so a
    // mutable borrow is fine again once we're past that point.
    let r2 = &mut s;
    r2.push('!');
    println!("{r2}");
}

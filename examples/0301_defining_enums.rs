// An enum defines a type by enumerating its possible variants. Unlike a
// struct, whose value always has the same fields, an enum value is exactly
// one of its variants - and each variant can optionally carry its own data.

enum IpAddrKind {
    V4,
    V6,
}

// Variants can carry data directly, so there's no need for a separate
// struct alongside the enum to hold each kind's payload.
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Variants can even carry struct-like named fields.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    variants_without_data();
    variants_carrying_data();
    variants_with_struct_like_fields();
}

fn variants_without_data() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
}

fn route(_kind: IpAddrKind) {}

fn variants_carrying_data() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    print_ip(&home);
    print_ip(&loopback);
}

fn print_ip(addr: &IpAddr) {
    match addr {
        IpAddr::V4(a, b, c, d) => println!("v4: {a}.{b}.{c}.{d}"),
        IpAddr::V6(s) => println!("v6: {s}"),
    }
}

fn variants_with_struct_like_fields() {
    describe(&Message::Quit);
    describe(&Message::Move { x: 10, y: 20 });
    describe(&Message::Write(String::from("hello")));
    describe(&Message::ChangeColor(255, 0, 0));
}

fn describe(msg: &Message) {
    match msg {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to ({x}, {y})"),
        Message::Write(text) => println!("write: {text}"),
        Message::ChangeColor(r, g, b) => println!("change color to ({r}, {g}, {b})"),
    }
}

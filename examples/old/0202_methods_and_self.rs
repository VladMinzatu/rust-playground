// Methods live in an `impl` block and take some form of `self` as their
// first parameter, describing how they interact with the instance:
// `&self` reads it, `&mut self` mutates it, `self` consumes it.

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle { width: side, height: side }
    }
}

fn main() {
    reading_with_shared_self();
    comparing_two_instances();
    mutating_with_mut_self();
    consuming_self_to_transform();
}

fn reading_with_shared_self() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("area: {}", rect.area()); // &self borrows rect, doesn't consume it
    println!("still usable: {}x{}", rect.width, rect.height);
}

fn comparing_two_instances() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
}

fn mutating_with_mut_self() {
    let mut rect = Rectangle { width: 10, height: 20 };
    rect.scale(2); // requires `rect` to be `mut`
    println!("scaled: {}x{}", rect.width, rect.height);
}

fn consuming_self_to_transform() {
    let rect = Rectangle { width: 10, height: 20 };
    let square = rect.into_square(); // `rect` is moved into `into_square`
    // println!("{}", rect.width); // error[E0382]: borrow of moved value: `rect`
    println!("square: {}x{}", square.width, square.height);
}

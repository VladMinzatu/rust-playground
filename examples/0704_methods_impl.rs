pub struct Queue {
    items: Vec<i64>
}

impl Queue {
    pub fn new() -> Self { // associated function, not a method, because it doesn't take &self or &mut self
        Queue { items: Vec::new() }
    }

    pub fn push(&mut self, item: i64) { // method, because it takes &mut self
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<i64> { // method, because it takes &mut self
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
}

fn just_hanging_around() { // this is a "free function", btw 
    println!("Just hanging around");
}
fn main() {
    let mut queue = Queue::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    println!("Popped: {:?}", queue.pop());
    println!("Popped: {:?}", queue.pop());

    just_hanging_around();
}
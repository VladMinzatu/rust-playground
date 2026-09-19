// Generics with trait bounds (`impl Trait`, `<T: Trait>`) are resolved at
// compile time: one specialized version per concrete type, no runtime
// cost. A trait object (`dyn Trait`) is the opposite trade-off - one
// function or collection can hold *different* concrete types behind a
// shared interface, decided at runtime, at the cost of a vtable lookup per
// call.

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        self.headline.clone()
    }
}

struct Tweet {
    username: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("tweet from @{}", self.username)
    }
}

fn main() {
    a_heterogeneous_collection_needs_trait_objects();
    dyn_reference_for_a_single_borrowed_value();
}

fn a_heterogeneous_collection_needs_trait_objects() {
    // `Vec<T>` needs one concrete `T` - a plain `Vec<Article>` couldn't also
    // hold a `Tweet`. `Box<dyn Summary>` erases the concrete type down to
    // "something that implements Summary", so different types can live
    // side by side in the same vector.
    let items: Vec<Box<dyn Summary>> = vec![
        Box::new(Article {
            headline: String::from("Rust 2.0 announced"),
        }),
        Box::new(Tweet {
            username: String::from("rustlang"),
        }),
    ];

    for item in &items {
        println!("{}", item.summarize()); // dispatched at runtime via the trait object's vtable
    }
}

fn notify(item: &dyn Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn dyn_reference_for_a_single_borrowed_value() {
    let article = Article {
        headline: String::from("Rust 2.0 announced"),
    };
    // `&dyn Trait` works the same way for a single borrowed value - no
    // `Box` needed when there's no ownership to transfer.
    notify(&article);
}

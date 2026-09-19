// A trait defines shared behavior as a set of method signatures. Any type
// can implement a trait by providing bodies for those methods, and a trait
// method can supply a default body that implementors are free to override.

trait Summary {
    fn author(&self) -> String;

    // A default implementation: types can use this as-is instead of
    // writing their own.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.author())
    }
}

struct Article {
    headline: String,
    author: String,
}

impl Summary for Article {
    fn author(&self) -> String {
        self.author.clone()
    }

    // Overrides the default entirely.
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

struct Tweet {
    username: String,
}

impl Summary for Tweet {
    fn author(&self) -> String {
        format!("@{}", self.username)
    }
    // No `summarize` override: uses the trait's default implementation.
}

fn main() {
    overridden_default_method();
    inherited_default_method();
}

fn overridden_default_method() {
    let article = Article {
        headline: String::from("Rust 2.0 announced"),
        author: String::from("Ferris"),
    };
    println!("{}", article.summarize());
}

fn inherited_default_method() {
    let tweet = Tweet {
        username: String::from("rustlang"),
    };
    println!("{}", tweet.summarize()); // falls back to Summary::summarize's default body
}

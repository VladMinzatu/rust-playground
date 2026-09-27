// A function can accept "some type that implements this trait" without
// knowing which concrete type ahead of time - as a parameter with `impl
// Trait` or an equivalent trait-bound generic, and (with one restriction)
// as a return type too.

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

// `impl Trait` sugar: reads nicely, but is really just shorthand for the
// generic-with-bound version below.
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The desugared, fully generic form - identical behavior to `notify` above.
// Prefer `impl Trait` for simple cases and switch to this form when you
// need to name the type parameter (e.g. to use it more than once).
fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple bounds with `+`: `item` must implement both traits.
fn notify_loudly(item: &(impl Summary + std::fmt::Debug)) {
    println!("Breaking news! {} ({:?})", item.summarize(), item);
}

#[derive(Debug)]
struct LoudTweet {
    username: String,
}

impl Summary for LoudTweet {
    fn summarize(&self) -> String {
        format!("tweet from @{}", self.username)
    }
}

// Returning `impl Trait` hands back "some type that implements Summary"
// without naming it - handy for e.g. closures or otherwise unnameable
// types. The catch: every path through the function must return the *same*
// concrete type, since under the hood this still isn't dynamic dispatch.
fn create_tweet(username: &str) -> impl Summary {
    Tweet {
        username: username.to_string(),
    }
}

fn main() {
    impl_trait_parameter();
    equivalent_generic_bound();
    multiple_trait_bounds();
    impl_trait_return_type();
}

fn impl_trait_parameter() {
    notify(&Article {
        headline: String::from("Rust 2.0 announced"),
    });
}

fn equivalent_generic_bound() {
    notify_generic(&Tweet {
        username: String::from("rustlang"),
    });
}

fn multiple_trait_bounds() {
    notify_loudly(&LoudTweet {
        username: String::from("ferris"),
    });
}

fn impl_trait_return_type() {
    let tweet = create_tweet("rustlang");
    println!("{}", tweet.summarize());
}

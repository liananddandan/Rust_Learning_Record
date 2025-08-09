use std::fmt::{Debug, Display};

// Define a trait with a required method and a default method.
pub trait Summary {
    fn summarize(&self) -> String;          // required method
    fn summarize_short(&self) -> String {   // default method
        String::from("(read more...)")
    }
}

pub struct News {
    pub title: String,
    pub author: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

// Trait bound on a function: accepts any T implementing Summary and Display.
pub fn notify<T: Summary + Display>(item: &T) {
    println!("[notify] {} | {}", item, item.summarize());
}

// The same bounds but using a `where` clause for readability.
pub fn pair_and_print<T, U>(t: &T, u: &U)
where
    T: Summary + Display,
    U: Debug,
{
    println!("pair_and_print: t={{}} -> {} | u={:?}", t, t.summarize(), u);
}

// Return type that implements a trait, without naming the concrete type.
pub fn make_summary() -> impl Summary {
    News { title: "Rust 1.XX Released".to_string(), author: "Ferris".to_string() }
}

use std::fmt;
impl fmt::Display for News {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "News('{}' by {})", self.title, self.author)
    }
}

pub fn run() {
    let n = News { title: "Generics in Rust".into(), author: "Alice".into() };
    println!("summarize: {}", n.summarize());
    println!("default summarize_short: {}", n.summarize_short());

    notify(&n);
    pair_and_print(&n, &42);

    let s = make_summary();
    println!("impl Trait value: {}", s.summarize());
}
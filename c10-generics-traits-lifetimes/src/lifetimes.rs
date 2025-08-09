// Lifetimes ensure that references are always valid. They do NOT change runtime behavior; they
// are purely compile-time checks. Most lifetimes are elided (inferred). We add explicit lifetimes
// when the compiler needs help relating the lifetimes of multiple references.

// Example: returning the longer of two &str slices. The returned &str must live at least as long
// as BOTH inputs. We name that common lifetime `'a`.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// A struct holding a reference must name a lifetime parameter for that field.
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // Methods often benefit from lifetime elision rules, so we don't need to write them explicitly.
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// Function demonstrating lifetime elision rules (inputs -> output relations).
// Rule of thumb: if there's exactly one input reference, the output gets the same lifetime.
pub fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(idx) => &s[..idx],
        None => s,
    }
}

pub fn run() {
    // Using `longest`
    let s1 = String::from("abcd");
    let s2 = String::from("xyz");
    let res = longest(&s1, &s2);
    println!("longest = {res}");

    // Using a struct with a referenced field (must outlive the struct)
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt { part: first_sentence };
    println!("excerpt.part = {}", excerpt.part);
    println!("announce: {}", excerpt.announce_and_return_part("Lifetimes are compile-time only"));

    // Lifetime elision demo
    let word = first_word("hello world");
    println!("first_word = {word}");

    // 'static demo: string literals have 'static lifetime
    let static_str: &'static str = "I live for the entire program";
    println!("static: {}", static_str);

    // Common lifetime pitfall (DANGLING) — this does NOT compile; shown for learning only:
    // let r: &str;
    // {
    //     let s = String::from("temp");
    //     r = &s; //  `s` drops here, `r` would dangle
    // }
    // println!("{r}");
}
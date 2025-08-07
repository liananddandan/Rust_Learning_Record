// Chapter 7: Managing Projects with Packages, Crates, and Modules

mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;

fn main() {
    eat_at_restaurant();
}

fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    hosting::add_to_waitlist();
}

// Submodule of back_of_house: kitchen

pub fn cook_order() {
    println!("Cooking order...");
}

fn fix_incorrect_order() {
    cook_order();
    super::super::deliver_order(); // accessing root-level function
}

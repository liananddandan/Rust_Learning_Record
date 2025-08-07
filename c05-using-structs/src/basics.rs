// Demonstrates basic struct creation and usage

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn run() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("another@example.com");

    println!("User: {:?}", user1);

    let user2 = build_user(String::from("new@example.com"), String::from("newuser"));
    println!("User 2: {:?}", user2);

    let user3 = User {
        email: String::from("third@example.com"),
        ..user2
    };

    println!("User 3: {:?}", user3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

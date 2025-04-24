fn main() {
    let user1 = build_user(String::from("email@example.com"), String::from("user123"));

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{0}", user2.username);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

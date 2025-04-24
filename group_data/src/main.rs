fn main() {
    let user1 = build_user(String::from("email@example.com"), String::from("user123"));

    println!("{0}", user1.email);
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




fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: bool,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sing_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}




fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        usernam:e,
        email,
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

    println!("User1 email: {user1.email}");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}

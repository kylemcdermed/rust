




fn main() {
    let user1 = User {
        active: bool,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sing_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

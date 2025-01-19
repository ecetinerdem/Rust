fn main() {
    struct User {
        active: bool,
        name: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        active: true,
        name: String::from("Arzu"),
        email: String:: from("Bxu5f@example.com"),
        sign_in_count: 1,
    }
}

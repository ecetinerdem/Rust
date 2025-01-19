fn main() {
    struct User {
        active: bool,
        name: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        active: true,
        username: String::from("someone123"),
        email: String:: from("Bxu5f@example.com"),
        sign_in_count: 1,
    }

    let mut user2 = User {
        active: true,
        username: String::from("someone321"),
        email: String:: from("someone321@example.com"),
        sign_in_count: 1,
    }

    
}

fn build_user (email: String, username: String) -> User {
    User {
        active: false,
        username,
        email,
        sign_in_count: 1,
    }
}



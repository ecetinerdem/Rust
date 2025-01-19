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

    let user3 = User {
        active: true,
        username: String::from("everyone"),
        email: String::from("everyone@example"),
        sign_in_count: 1,
    }

    let user4 = User {
        active: user3.active,
        username: user3.username,
        email: String::from("everyone@example"),
        sign_in_count: user3.sign_in_count,
    }

    let user5 = USer {
        email: String::from("everybody@example"),
        ..user3
    }


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    

    let subject = AlwaysEqual;
}

fn build_user (email: String, username: String) -> User {
    User {
        active: false,
        username,
        email,
        sign_in_count: 1,
    }
}


struct Color (i32, i32, i32);
struct Point (i32, i32, i32);


struct AlwaysEqual;
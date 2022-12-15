struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        email, // shorthand to => email: email,
        username, // shorthand to => username: username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    let user1 = User {
        email: String::from("somone@example.com"),
        username: String::from("somusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Creating Instances From Other Instances With Struct Update Syntax
    let user3 = User {
        email: String::from("yetanother@example.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0,);

    let subject = AlwaysEqual;

}

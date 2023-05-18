pub fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
      email: String::from("another@example.com"),
      ..user1
  };

    println!("user1: {}", user1.email);
    println!("user2: {}", user2.email);
    println!("user1: {}", user1.active);
    // println!("user1: {}", user1.username);  // ERROR: user1.username is moved to user2.username

    let black = Color(0, 0, 0);

    let subject = AlwaysEqual;
}

// Unit Structs
struct AlwaysEqual;

// Tuple Structs
struct Color(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }

    // Shorthand syntax
    // User {
    //     active: true,
    //     username,
    //     email,
    //     sign_in_count: 1,
    // }
}

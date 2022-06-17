use std::cmp::Ordering;

struct User {
    name: String,
    email: String,
    age: u8,
    active: bool,
}

struct Color (i32, i32, i32);
struct Coordinate (i32, i32, i32);

fn new_user(name: String, email: String, age: u8) -> User {
    User { name, email, age, active: true }
}

fn main() {
    let user = User{
        name: String::from("VoN"),
        email: String::from("sudo.von.contact@gmail.com"),
        age: 25,
        active: true,
    };
    println!("User name: {}", user.name);
    let another_user = new_user(String::from("VoN-2"), String::from("sudo.von.contact.2@gmail.com"), 25);
    let another_user_2 = User {
        name: String::from("VoN"),
        email: String::from("sudo.von.contact@gmail.com"),
        ..another_user
    };
    println!("User name: {}", another_user.name);
    println!("User name: {}", another_user_2.name);

    let black = Color(0,0,0);
    let origin = Coordinate(0,0,0);
}

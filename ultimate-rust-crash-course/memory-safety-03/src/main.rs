fn main() {
    let enigma: i32;
    if true {
        enigma = 42;
    }
    println!("{}", enigma); // Error!
}

fn main() {
    let enigma: i32;
    if true {
        enigma = 42;
    } else {
        enigma = 22;
    }
    println!("{}", enigma); // Error!
}

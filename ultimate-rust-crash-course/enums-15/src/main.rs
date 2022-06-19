use std::fs::File;

enum Color {
    Red,
    Green,
    Blue,
}

enum DispenserItem {
    Empty,
    Ammo(u8),
    Things(String, i32),
    Place {x: i32, y: i32},
}

fn main() {
    let color = Color::Blue;

    match color {
        Color::Red => {
            println!("It's red");
        },
        Color::Green => {
            println!("It's green");
        },
        _ => {
            println!("Everything")
        }
    }

    let mut x: Option<i32> = None;
    x = Some(5);
    x.is_some();
    x.is_none();

    let res = File::open("foo");
    match res {
        Ok(f) => {
            /* do stuff */
        },
        Err(e) => {
            /* do stuff */
        }
    }
}

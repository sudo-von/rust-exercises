fn main() {
    let x = 5;
    {
        let y = 99;
        println!("{} - {}", x, y);
    }
    println!("{} - {}", x, y); // Error!
}

fn shadowing() {
    let x = 5;
    {
        let x = 99;
        println!("{}", x); // Prints "99"
    }
    println!("{}", x); // Prints "5"
}

fn shadowing_different_typw() {
    let meme = "More cowbell!";
    let meme = make_image(meme);
}

fn main() {
    let str = String::from("Hello friend");
    let word = first_word(&str);
    println!("First word: {}", &word);
}

fn first_word(string: &String) -> &str {
    let bytes = string.as_bytes();
    for (i, &item) in bytes.iter().enumerate()    {
        if item == b' ' {
            return &string[0..i];
        }
    }
    &string[..]
}
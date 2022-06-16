fn main() {
    let s = String::from("Hello friend");
    let lon = calculate_lon(&s);
    println!("Longitude: {}", lon);

    let mut p = String::from("Hello");
    update_string(&mut p);
    println!("{}", p);
}

fn calculate_lon(string: &String) -> usize {
    let aux = string.len();
    aux
}

fn update_string(string: &mut String) {
    string.push_str(" friend");
}
fn main() {
    let k = String::from("🥝");
    let f = move || {
        print!("{}", k);
    };
    f();
}

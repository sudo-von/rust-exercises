fn main() {
    let s1 = String::from("abc");
    do_stuff(&s1);
}

fn do_stuff(s: &String) {
    println!("{}", s);
}

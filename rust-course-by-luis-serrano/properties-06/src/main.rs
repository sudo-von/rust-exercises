fn main() {
    /* Stack. */
    let a = 10;
    let b = a;
    println!("a : {}", a);
    println!("b : {}", b);
    /* Heap. */
    let c = String::from("Hello, world!");
    let d = c.clone();
    println!("c : {}", c);
    println!("d : {}", d);
}
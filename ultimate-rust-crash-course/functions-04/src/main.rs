fn main() {
    let stuff = do_stuff(20.0, 10.0);
    println!("{}", stuff);
}

fn do_stuff(qty: f64, oz: f64) -> f64 {
    qty + oz
}
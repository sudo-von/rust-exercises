use std::{hash::Hash, collections::HashMap};

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    let x = v.pop();
    println!("{}", v[1]);

    let mut y = vec![5,6,7];

    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(5, true);
    let have_five = h.remove(&5).unwrap();

}

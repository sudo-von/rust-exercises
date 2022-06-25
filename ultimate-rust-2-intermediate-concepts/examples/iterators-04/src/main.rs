fn main() {
    let vec = vec![6, 7, 8];
    let vec2 = vec.clone();

    let another_vector = vec2
        .into_iter()
        .map(|x: i32| x * 3)
        .filter(|y: &i32| *y % 2 == 0)
        .collect::<Vec<_>>();

    let total: i32 = vec
        .into_iter()
        .map(|x: i32| x * 3)
        .filter(|y: &i32| *y % 2 == 0)
        .sum();

    println!("{}", total);

    /*
        v.into_iter() // consumes v, returns owned items for _ in v
        v.iter() // returns inmutable references for _ in &v
        v.iter_mut() // returns mutable references for _ in &mut v
    */
}

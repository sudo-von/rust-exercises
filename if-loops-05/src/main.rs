fn main() {
    /* Conditional. */
    let number = 10;
    if number < 10 {
        println!("{} is lower than 10", number);
    } else if number > 10 {
        println!("{} is higher than 10", number);
    } else {
        println!("{} is equal to 10", number);
    }
    /* Expression. */
    let is_valid = false;
    let number = if is_valid { 10 } else { 20 };
    println!("{}", number);

    /* Loops. */
    let mut i = 0;
    let result = loop {
        println!("{}", i);
        if i == 10 {
            break i;
        };
        i += 1;
    };

    println!("Your result is: {}", result);

    let mut j = 0;
    while j != 0 {
        println!("{}", j);
        j -= 1;
    }

    let matrix = [1,2,3,4,5,6,7,8,9];
    let mut k = 0;
    while k < matrix.len() {
        println!("k - {}", matrix[k]);
        k += 1;
    }

    for num in matrix.iter() {
        println!("num - {}", num);
    }

    for num in 1..10 {
        println!("range - {}", num);
    }

    for num in (1..10).rev() {
        println!("rev - {}", num);
    }
}

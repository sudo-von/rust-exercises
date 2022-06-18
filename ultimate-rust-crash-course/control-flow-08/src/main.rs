fn main() {
    let num = 10;

    let first_message;
    if num == 5 {
        first_message = "five";
    } else if num == 4 {
        first_message = "four";
    } else {
        first_message = "other";
    }

    println!("{}",first_message);

    let second_message = if num == 5 {
        "five"
    } else if num == 4 {
        "four"
    } else {
        "other"
    };

    println!("{}", second_message);

    let mut num = 10;
    'bob: loop {
        loop {
            num += 1;
            loop {
                if num < 15 {
                    continue 'bob;
                } else {
                    break 'bob;
                }
            }
        }
    }

    println!("{}", num);

    for n in [7,8,9].iter() {
        println!("{}", n);
    }

    let array = [(1,2), (3,4)];
    for (x,y) in array.iter() {
        println!("{} - {}", x,y);
    }

    for n in 0..50 {
        println!("{}", n);
    }

    for n in 0..=50 {
        println!("{}", n);
    }

}

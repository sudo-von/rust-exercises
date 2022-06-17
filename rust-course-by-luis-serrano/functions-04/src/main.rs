fn main() {
    /* Function without params. */
    hello_world();
    /* Function with params. */
    get_number(50);
    /* Expression. */
    let expression = {
        let number = 10 + 20;
        number + 1
    };
    println!("Your expression is: {}", expression);
    println!("Your result is: {}", sum(1, 2));
    println!("Your result is: {}", sum_with_return(1, 2));
}

fn hello_world() {
    println!("Hello, world!");
}

fn get_number(number: i32) {
    println!("Your number is: {}", number);
}

fn sum(first_number: i8, second_number: i8) -> i8 {
    first_number + second_number
}

fn sum_with_return(first_number: i8, second_number: i8) -> i8 {
    return first_number + second_number;
}
fn main() {
    /* Variables. */
    let number = 1;
    println!("The value from number is: {}", number);
    /* Mutable variables. */
    let mut mutable_number = 2;
    println!("The value from mutable_number is: {}", mutable_number);
    mutable_number = 3;
    println!("The new value from mutable_number is: {}", mutable_number);
    /* Constants. */
    const PI: f32 = 3.1416;
    println!("The value from the constant PI is: {}", PI);
    /* Shadowing. */
    let shadowing_variable = 4;
    println!("The value from the variable shadowing_variable is: {}", shadowing_variable);
    let shadowing_variable = "VoN";
    println!("The new value from the variable shadowing_variable is: {}", shadowing_variable);
}

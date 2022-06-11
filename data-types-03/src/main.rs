fn main() {
    /* Scalars.
     * * Integer.
     * * Float.
     * * Boolean.
     * * Characters.
    */

    /*
     * Integers.
     * * 8-bit   i8    u8
     * * 16-bit  i16   u16
     * * 32-bit  i32   u32
     * * 64-bit  i64   u64
     * * 128-bit i128  u128
     * *         isize usize
    */
    let inumber:i8 = -1;
    let unumber:u8 = 1;
    println!("i8: {} - u8: {}", inumber, unumber);

    /* Floats. */
    let f32_decimal:f32 = 0.0;
    let f64_decimal:f64 = 1.0;
    println!("f32: {} - f64: {}", f32_decimal, f64_decimal);

    /* Booleans. */
    let is_true:bool = true;
    let is_false:bool = false;
    println!("true: {} - false: {}", is_true, is_false);

    /* Characters.
     * Uses 4 bytes.
    */
    let emoji:char = '😱';
    println!("char: {}", emoji);

    /* Compounds.
     * * Tuples.
     * * Arrays.
    */
    let tuple:(f32,u8,f64,i8) = (500.30, 80, -40.25, -80);
    let (first_number, second_number, third_number, fourth_number) = tuple;
    println!("tuple: {} - {} - {} - {}", first_number, second_number, third_number, fourth_number);

}

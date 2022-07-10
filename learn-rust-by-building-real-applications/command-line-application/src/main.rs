fn main() {
    let mut margs_weight: f32 = calculate_weight_on_mars(100.0);
    margs_weight = margs_weight * 1000.0;
    println!("Weight on Mars: {} kg", margs_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}



pub convert_float_values() {
    let decimal_value: f32 = 2.5;

    let rounded_value = decimal_value.round();
    let floored_value = decimal_value.floor();

    println!("Value: {}", decimal_value); // Output: Value: 2.5
    println!("Rounded value: {}", rounded_value); // Output: Rounded value: 3.0
    println!("Floored value: {}", floored_value); // Output: Floored value: 2.0
    
}
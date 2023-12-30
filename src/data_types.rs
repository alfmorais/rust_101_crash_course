pub fn data_types() {
    println!("Data Types in Rust:");

    let is_active: bool = true;
    let quantity: i32 = 10;
    let price: f64 = 19.99;
    let character: char = 'A';
    let phase = "Hello World!".to_string();

    println!("Boolean: {}", is_active);
    println!("Integer: {}", quantity);
    println!("Double or Float: {}", price);
    println!("Character: {}", character);
    println!("String: {}", phase);
}

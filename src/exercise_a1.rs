// Topic: Functions
//
// Program requirements:
// * Display your first and last name
// 
// Notes:
// * Use a function to display your first_name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

// 1º Solution
pub fn my_name(first_name: String, last_name: String) -> () {
    println!("{} {}", first_name, last_name);
    println!("First Name: {}", first_name);
    println!("Last Name: {}", last_name);
}

pub fn first_name() {
    println!("Alfredo");
}

pub fn last_name() {
    println!("de Morais Neto");
}
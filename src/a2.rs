/*
TOPIC: Basic Arithmetic

Program requirements;
* Displays the result of the sum of two numbers

Notes:
* Use a function to add two number together
* Use a function to display the result
* Use the "{:?}" token in the println macro to display the result

*/

pub fn add(first_number: u32, second_number: u32) -> u32 {
    return first_number + second_number;
}

pub fn display_result(total: u32) -> () {
    println!("A soma é: {:?}", total);
}

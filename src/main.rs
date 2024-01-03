mod basic_arithmetic;
mod control_flow_if;
mod data_types;
mod a1;
mod a2;
mod a3;
mod exercise_a3b;
mod a4;
mod a5;
mod function;
mod looping;
mod macro_println;
mod match_function;
mod variables;

fn main() {
    println!("Hello, world!");

    data_types::data_types();

    variables::variables();

    let x = function::add(1, 2);
    println!("X = {}", x);

    macro_println::macro_println();

    control_flow_if::control_flow_if(17);

    looping::looping_for();
    looping::looping_while();

    let first_name = "Alfredo".to_string();
    let last_name = "De Morais Neto".to_string();
    a1::my_name(first_name, last_name);
    a1::first_name();
    a1::last_name();

    basic_arithmetic::basic_operations();

    let total = a2::add(10, 9);
    a2::display_result(total);

    let is_arrived = true;
    a3::greetings(is_arrived);

    let is_arrived = false;
    a3::greetings(is_arrived);

    let number: i32 = 5;
    a3::check_number_logic(number);

    match_function::match_example();
    match_function::match_my_name();

    let some_bool = false;
    a4::checking_boolean(some_bool);

    let some_integer = 2;
    a4::checking_number(some_integer);

    a5::looping();
}

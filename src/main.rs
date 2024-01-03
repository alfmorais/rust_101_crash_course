mod basic_arithmetic;
mod control_flow_if;
mod data_types;
mod exercise_a1;
mod exercise_a2;
mod exercise_a3;
mod exercise_a3b;
mod excercise_a4;
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
    exercise_a1::my_name(first_name, last_name);
    exercise_a1::first_name();
    exercise_a1::last_name();

    basic_arithmetic::basic_operations();

    let total = exercise_a2::add(10, 9);
    exercise_a2::display_result(total);

    let is_arrived = true;
    exercise_a3::greetings(is_arrived);

    let is_arrived = false;
    exercise_a3::greetings(is_arrived);

    let number: i32 = 5;
    exercise_a3b::check_number_logic(number);

    match_function::match_example();
    match_function::match_my_name();

    let some_bool = false;
    excercise_a4::checking_boolean(some_bool);

    let some_integer = 2;
    excercise_a4::checking_number(some_integer);
}

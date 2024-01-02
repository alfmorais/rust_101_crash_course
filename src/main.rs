mod basic_arithmetic;
mod control_flow_if;
mod data_types;
mod exercise_a1;
mod exercise_a2;
mod function;
mod looping;
mod macro_println;
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
}

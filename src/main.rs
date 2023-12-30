mod control_flow_if;
mod data_types;
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
}

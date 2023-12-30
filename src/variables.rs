// Disable line 9 warning -> let mut my_name: &str = "Bill";
#[allow(unused_mut)]

pub fn variables() {
    let two = 2;
    let hello = "hello";
    let j = 'j';
    let my_half = 0.5;
    let mut my_name = "Bill";
    let quit_program = false;
    let your_half = my_half;

    println!(
        "Integer: {}, String: {}, Char: {}, Mutable String: {}, Boolean: {} and Float/Double: {}.",
        two, hello, j, my_name, quit_program, your_half
    )
}

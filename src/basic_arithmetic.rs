pub fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

pub fn basic_operations() {
    let sum = 2 + 2;
    let value = 10 - 2;
    let division = 10 / 2;
    let mult = 5 * 5;

    let _five = sub(8, 3);

    let rem = 6 % 3;
    let rem_ = 6 % 4;

    println!(
        "Sum: {} \n\
        Difference: {} \n\
        Division: {} \n\
        Multiplication: {} \n\
        Remander (6 % 3): {} \n\
        Remander (6 % 4): {} \n\
        ",
        sum, value, division, mult, rem, rem_
    )
}

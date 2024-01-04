/*
TOPIC: WORKING WITH EXPRESSIONS

REQUIREMENTS:
- Print "it's big" if a variable is > 100
* Print "it's small" if a variable is <= 100

NOTES:
- Use a boolean variable set to an expression that determines whether the value is >100 or <=100
- Use a function to print the messages
- Use a match expression to determine which message to print
*/

pub fn its_big() -> () {
    println!("Its big");
}

pub fn its_small() -> () {
    println!("Its small");
}

pub fn result(value: i32) {
    let comparative_number = 100;
    let condition = if value > comparative_number { true } else { false };

    match condition {
        true => its_big(),
        false => its_small(),
    };
}
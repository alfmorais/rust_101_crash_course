/* 
TOPIC: DECISION MAKING WITH MATCH

PROGRAM REQUIREMENTS:
* Display "It's true" or "It's false" based on the value a boolean variable

NOTES:
* Use a variable set to either true or false
* Use a match expression to determine which message to display
*/


pub fn checking_boolean(some_bool: bool) -> () {
    match some_bool {
        true => println!("It's true"),
        _ => println!("It's false"),
    }
}

/* 
TOPIC: DECISION MAKING WITH MATCH

PROGRAM REQUIREMENTS:
* Display "one", "two", "three", or "other" base on whether
 the value of a variable is 1, 2, 3, or some other number, respectively

NOTES:
* Use a variable set to any integer
* Use a match expression to determine which message to display
* Use an underscore (_) to match on any value
*/
pub fn checking_number(some_integer: i32) -> () {
    match some_integer {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}

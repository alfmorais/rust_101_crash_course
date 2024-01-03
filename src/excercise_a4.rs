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
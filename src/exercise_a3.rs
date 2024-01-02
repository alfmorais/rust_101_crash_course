/*
TOPIC: Flow control using if..else

Program Requirements:
* Displays a message based on the value of a boolean variable
* When the variable is set to true, display "Hello"
* When the variable is set to false, display "GoodBye"

Notes: 
* Use a variable set to either true or false
* Use an if..else block to determine which message to display
* Use the println macro to display messages to the terminal
*/

pub fn greetings(is_arrived: bool) -> () {
    // Can be used like this: if is_arrived == true
    if is_arrived {
        println!("Hello");
    } else {
        println!("GoodBye");
    }
}
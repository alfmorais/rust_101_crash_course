/*
TOPIC: FLOW CONTROL USING IF..ELSE IF..ELSE

PROGRAM REQUIREMENTS:
* Display ">5", "<5", or "=5" based on the value of a variable
  is > 5, < 5, or == 5, respectively

NOTES:
* Use a variable set to any integer value
* Use an if..else if..else block to determine which message to display
* Use the println macro to display messages to the terminal
*/

pub fn check_number_logic(number: i32) -> () {
    if number > 5 {
        println!("O {number} é maior que '5'");
    } else if number < 5 {
        println!("O {number} é menor que '5'");
    } else {
        println!("Igual {number} a '5'");
    }
}

/*
TOPIC: LOOPING USING THE LOOP STATEMENT

PROGRAM REQUIREMENTS:
* Display "1" through "4" in the terminal

NOTES:
* Use a mutable integer variable
* Use a loop statement
* Print the variable within the loop statement
* Use break to exit the loop

*/

pub fn looping() -> () {
    let mut number = 1;

    loop {
        if number == 4 {
            println!("Finished at: {number:?}");
            break;
        }
        println!("Count: {:?}", number);
        number = number + 1;
    }
}

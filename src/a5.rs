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

/*
TOPIC: LOOPING USING THE WHILE STATEMENT

PROGRAM REQUIREMENTS:
* Count down from 5 to 1, display the countdown in the terminal, then prints "done!" when complete.

NOTES:
* Use a mutable integer variable
* Use a while statement
* Print the variable within the while loop
* Do not use break to exit the loop
*/

pub fn looping_with_while() -> () {
    let mut a = 5;

    while a != 0 {
        println!("Count: {a:?}");
        a = a - 1;
    }
    println!("Done!");
}

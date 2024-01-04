/*
TOPIC: WORKING WITH AN ENUM

PROGRAM REQUIREMENTS:
* Prints the name of a color to the terminal

NOTES:
* Use an enum with color name as variants
* Use a function to print the color name
* The function must use the enum as a parameter
* Use a match expression to determine which color name to print
*/

pub enum Colors {
    Blue,
    Red,
    Green,
    Black,
}

pub fn print_colors(colors: Colors) -> () {
    match colors {
        Colors::Blue => println!("Blue"),
        Colors::Red => println!("Red"),
        Colors::Green => println!("Green"),
        Colors::Black => println!("Black"),
    }
}

/*
TOPIC: Data management using tuples

REQUIREMENTS:
- Print whether the x-value of a cartesian coordinate is greater than 5, less than 5 or equal to 5.

NOTES:
- Use a function that returns a tuple
- Destructure the return value into two variables
- Use an if..else if..else block to determine to print
*/

pub fn coordinates() -> (i32, i32) {
    let coordinates_values = (8, 5);
    let x = coordinates_values.0;

    if x == 5 {
        println!("Equal to: 5 == {:?}", x);
    } else if x > 5 {
        println!("Greater than: {:?} > 5", x);
    } else {
        println!("Less than: {:?} < 5", x);
    }

    return coordinates_values;
}
/*
ENUMARATION

- Data that can be one of multiple different possibilities
-> Each possibility is called a "variant"

- Provides information about your program to the compiler
-> More robust program

RECAP:

* Enums can only be one variant at a time
* More robust programs when paired with match
* Make program code easier to read
*/

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn which_way(go: Direction) {
    match go {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}


pub fn main() {
    let go = Direction::Left;
    
    match go {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
        _ => println!("do nothing!"),
    }
}
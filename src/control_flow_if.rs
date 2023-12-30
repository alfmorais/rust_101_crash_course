// Example with nested if .. else condition

pub fn control_flow_if(age: i32) -> () {
    /* 
    Old example works, but was refactored!
    
    if age >= 18 {
        if age > 60 {
            println!("Is a Old Person")
        } else {
            println!("Is a Adult");
        }
    } else {
        println!("Is a child");
    }
    */

    if age >= 60 {
        println!("Is a Old Person");
    } else if age < 60 && age >= 18 {
        println!("Is a Adult");
    } else {
        println!("Is a child");
    }
}

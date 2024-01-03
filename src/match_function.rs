pub fn match_example(){
    let some_bool = true;
    
    match some_bool {
        true => println!("It's true"),
        false => println!("It's false"),
    }
}

pub fn match_my_name() {
    let my_name = "Alfredo";

    match my_name {
        "Alfredo" => println!("That is my name"),
        _ => println!("This is not my name"),
    }
}
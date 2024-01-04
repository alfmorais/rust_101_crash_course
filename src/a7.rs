/*
TOPIC: Organizing similar data using structs

REQUIREMENTS:
- Print the flavor of a drink and it's fluid ounces

NOTES:
- Use an enum to create different flavors of drinks
- Use a struct to store drink and fluid ounce information
- Use a function to print out the drink flavor and ounces
- Use a match expression to print the drink flavor
*/

pub enum Flavors {
    ChocolateLiquer,
    SparkingWine,
}

pub struct Drink {
    pub flavor: Flavors,
    pub fluid_oz: f64,
}

pub fn show_drink(drink: Drink) -> () {
    match drink.flavor {
        Flavors::ChocolateLiquer => println!("Flavor: Choclate Liquer"),
        Flavors::SparkingWine => println!("Flavor: Sparking Wine"),
    }
    println!("Oz: {:?}", drink.fluid_oz);
}

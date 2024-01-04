/*
Expressions

- Rust is an expression-based language
-> Most things are evaluated and return some value

- Expression values coalesce to a single point
-> Can be used for nesting logic

Recap:
- Expression allow nested logic
- if and match expression can be nested
-> Best to not use more than two or three levels
*/

pub enum Menu {
    Burger,
    Fries,
    Drink,
}

pub enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

pub fn expression_logic() -> () {
    // Example 1
    let number = 3;
    let is_lower_than_five = if number < 5 { true } else { false };
    let is_lower_than_five_second = number < 5;

    println!(
        "Number {:?}\n\
        Is Lower Than Five: {:?}\n\
        Is Lower Than Five: {:?}\n",
        number, is_lower_than_five, is_lower_than_five_second
    );

    // Example 2
    let paid = true;
    let item = Menu::Drink;
    let drink_type = "water";
    let order_placed = match item {
        Menu::Drink => if drink_type == "water" { true } else { false },
        _ => true,
    };

    // Example 3: secret file -> admins only
    let access_level = Access::Guest;
    let can_access_file = match  access_level {
        Access::Admin => true,
        _ => false,
    };
}

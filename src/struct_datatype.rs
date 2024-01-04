/*
STRUCTURE

- A type that contains multiple pieces of data:
-> All or nothing - cannot have some pieces of data and not others

- Each piece of data is called a "field"
- Makes working with data easier
-> Similar data can be grouped together
*/


#![allow(dead_code)]
pub struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

pub struct GroceryItem {
    stock: i32,
    price: f64,
}

pub fn struct_data() {
    let my_box = ShippingBox {
        depth: 3,
        width: 2,
        height: 5,
    };

    let tall = my_box.height;
    println!("The box is {:?} units tall", tall);
}

pub fn grocery_data() {
    let cereal = GroceryItem {
        stock: 10,
        price: 2.88,
    };

    println!("Stock: {:?}", cereal.stock);
    println!("Price: {:?}", cereal.price);
}
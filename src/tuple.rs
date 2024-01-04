#[derive(Debug)]
pub enum Access {
    Full,
}

pub fn one_two_three() -> (i32, i32, i32) {
    return (1, 2, 3);
}

pub fn function_tuple_demo() -> () {
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();

    println!("{:?}, {:?}", x, numbers.0);
    println!("{:?}, {:?}", y, numbers.1);
    println!("{:?}, {:?}", z, numbers.2);

    let (employee, access) = ("Jack", Access::Full);
    println!("Employee: {:?} | Access: {:?}", employee, access);
}
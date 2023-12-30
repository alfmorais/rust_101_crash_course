pub fn macro_println() {
    let life = 42;

    println!("hello");
    println!("{:?}", life);
    println!("{:?} {:?}", life, life);
    println!("The meaning is {:?}", life);
    println!("{life:?}");
    println!("{life}");
}
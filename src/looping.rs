pub fn looping_for() {
    let mut a = 0;

    loop {
        if a == 5 {
            println!("Finished at: {a:?}");
            break;
        }
        println!("{:?}", a);
        a = a + 1;
    }
}

pub fn looping_while() {
    let mut a = 0;

    while a != 5 {
        println!("{a:?}");
        a = a + 1;
    }
    println!("Finished at: {a:?}");
}

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

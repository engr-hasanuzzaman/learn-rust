pub fn run() {
    // intifinte loop
    let mut count = 0;

    loop {
        count += 1;
        println!("The value {}", count);
        if count == 20 {
            break;
        }
    }
}
pub fn run() {
    // intifinte loop
    let mut count = 1;

    // loop {
    //     count += 1;
    //     println!("The value {}", count);
    //     if count == 20 {
    //         break;
    //     }
    // }

    // while loop
    while count  <= 100 {
        if count % 15 == 0 {
            println!("FizBuzz");
        } else if count % 3 == 0 {
            println!("Fiz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count)
        }

        count += 1;
    }
}
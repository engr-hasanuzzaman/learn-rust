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
    // while count  <= 100 {
    //     fiz_buzz(count);
    //     count += 1;
    // }

    // range loop
    for count in 1..100 {
        fiz_buzz(count);
    }
}

fn fiz_buzz(count: i32) {
    if count % 15 == 0 {
        println!("FizBuzz");
    } else if count % 3 == 0 {
        println!("Fiz");
    } else if count % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", count)
    }
}
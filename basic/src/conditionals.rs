pub fn run() {
    let age = 22;
    let id_checked = false;

    if age >= 21 && id_checked {
        println!("What do you want to drink?");
    } else if age >= 21 && !id_checked {
        println!("Please need to see your id");
    } else {
        println!("You should not come here");
    }

    let is_tenager = if age >= 20 {true} else {false};
    println!("Are you teenager {}", is_tenager);
}
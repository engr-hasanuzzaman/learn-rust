// variable holds primitive data or reference
// by default variables are immutable
// variables are block scope

pub fn run() {
    let name = "hasan";
    let mut age = 30;
    println!("My name is {} and I am {}", name, age);
 
    // change variable value
    age = 32;
    println!("My name is {} and I am {}", name, age);
}
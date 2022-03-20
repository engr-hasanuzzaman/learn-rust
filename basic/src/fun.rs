pub fn run() {
    greeting("Hello", "Max");
    let sum = get_sum(10, 15);
    println!("The sum of 10 & 15 is {}", sum);

    // use closure
    let add_sum = |n1: i32, n2: i32| n1 + n2;
    println!("The sum of 10 & 15 using closure {}", add_sum(10, 15));

}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn get_sum(n1: i32, n2: i32) -> i32 {
    // skip semicolon means this is our return value
    // other wise have to use return n1 + n2;
    n1 + n2
}
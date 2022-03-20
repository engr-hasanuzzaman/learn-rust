pub fn run() {
    let person: (&str, &str, i32) = ("Max", "Marse", 21);

    println!("{} is from the {} and {}", person.0, person.1, person.2);
}
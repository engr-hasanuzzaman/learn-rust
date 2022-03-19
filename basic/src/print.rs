pub fn run() {
    // basic printing
    println!("This is basic printing with newline");

    // print with formatting
    println!("Hello {} on the {} world", "Dev", "Rust");

    // print with position
    println!("Wellcome {0} to rust world. How old are you {0}?", "Foo");

    // print with named argument
    println!("My name is {name} and I am {age} year old", name = "Hasan", age = 32);

    // Placeholder trait
    println!("Binary is {0:b}, Hex {0:x}, Octal {0:o}", 10);
}
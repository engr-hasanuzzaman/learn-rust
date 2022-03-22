struct Person {
    name: String,
    age: i32,
}

pub fn run() {
    let p = Person {
        name: String::from("Mac"),
        age: 20
    };

    println!("The struct is {}, age {}", p.name, p.age);
}
struct Person {
    name: String,
    age: i32,
    gender: String
}

impl Person {
    fn new(name: &str, age: i32, gender: &str) -> Person{
        Person {
            name: name.to_string(),
            age,
            gender: gender.to_string(),
        }
    }

    fn category(&self) -> &str {
        if self.age > 10 {
            "Matured"
        } else {
            "Teen"
        }
    }
}


pub fn run() {
    let p = Person::new("Mac", 32, "male");

    println!("The struct is {}, age {}, gender: {}", p.name, p.age, p.gender);
    println!("Persons category is {}", p.category());
}
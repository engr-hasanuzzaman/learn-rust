// two type of strings, immutable and mutable heap allocated grawable datastructure

pub fn run() {
    let hello = "Hello World"; // immutable string
    let mut expandable_string = String::from("This is the beginning");
    // two ways to add additional on top of expandable string
    expandable_string.push_str(" with ending ");
    expandable_string.push('\u{1F600}');
    let size = expandable_string.len();
    println!("{:?}", (hello, expandable_string, size));
}

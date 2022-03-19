// statically type programming language but we can use implicit type based on the
// initial value 
// i8, i32......., f32, f64, Array, tuple, char, Boolean
// rust flow snake case for variable naming
pub fn run() {
    let unicode_char = "\u{1F600}";
    println!("{:?}", (std::i32::MAX, std::f64::MAX, unicode_char));
}

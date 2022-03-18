use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first_num = args.nth(1).unwrap();
    // since args contains iterator we want to access the next available elemen
    let operator = args.nth(0).unwrap();
    let second_num = args.nth(0).unwrap();
    print!("{} {} {} = {}", first_num, operator, second_num, 420);
}

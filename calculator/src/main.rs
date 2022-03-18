use std::env::{args, Args};

fn main() {
    let args: Args = args();
    print!("Your command line input is {:?}", args);
}

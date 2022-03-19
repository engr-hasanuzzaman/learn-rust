use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first = args.nth(1).unwrap();
    // since args contains iterator we want to access the next available elemen
    let operator = args.nth(0).unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let operator = operator.chars().next().unwrap();
    let result = operate(operator, first_number, second_number);

    print!("{:?}", output(operator, first_number, second_number, result));
}

fn operate(operator: char, first_num: f32, second_num: f32) -> f32 {
    if operator == '+' {
        first_num + second_num
    } else if  operator == '-' {
        first_num - second_num
    } else if operator == '/' {
        first_num / second_num
    } else if operator == '*' {
        first_num * second_num
    } else {
        0.0
    }
}

fn output(operator: char, first_num: f32, second_num: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_num, operator, second_num, result)
}

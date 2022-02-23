use std::env::{args, Args};

// cargo run -- 1 + 2

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    println!(
        "println called\n\tresult: {} {} {}",
        first, operator, second
    );
}

fn operate(operator: char, first_number: f32, second_number: f32) {}

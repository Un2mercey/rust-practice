use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first = args.nth(0);
    println!("println called: {:?}", first);
}

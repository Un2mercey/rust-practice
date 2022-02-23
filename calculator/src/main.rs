use std::env::{args, Args};

// cargo run -- 1 + 2

fn main() {
    let mut args: Args = args();
    let first = args.nth(1).unwrap();
    let operator = args.nth(2).unwrap();
    let second = args.nth(3).unwrap();
    println!("println called: {:?} {} {}", first, operator, second);
}

fn nth(&mut self, n: usize) -> Option<String> {
    // assume n = 0;
    // inner = ["1", "2"]
    self.inner.next(); // "1"
                       // calling next() again result in second element
    self.inner.next() //"2"
}

mod args;
use args::Args;

fn main() {
    let args = Args::new();
    println!("`println!` called\n\t{:?}", args);
}

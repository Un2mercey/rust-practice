mod args;
use args::Args;

fn main() {
    let args = Args::new();
    let println!("`println!` called\n\t{:?}", args);
}

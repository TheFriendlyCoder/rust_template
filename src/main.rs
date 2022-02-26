// Treat all enabled lints as errors
#![deny(clippy::all)]

fn main() {
    let val = true;
    if val {
        println!("Hello, world!");
        }
}

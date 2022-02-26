#![deny(clippy::all)]

fn main() {
    let fubar = false;
    if !fubar {
        println!("Hello, world2!");
    }
}

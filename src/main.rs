#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

fn main() {
    let fubar = false;
    if fubar == false {
        println!("Hello, world2!");
    }
}

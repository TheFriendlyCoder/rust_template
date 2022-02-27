use std::{error::Error, fmt::Debug};

type MyResult<T> = Result<T, Box<dyn Error>>;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

pub fn run() -> MyResult<()> {
    let config = Args::parse();
    dbg!(config);
    Ok(())
}

use crate::models::Polymerization;
use std::error::Error;
use std::process::exit;
use std::{env, fs};

mod models;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let file_content =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");
    let input = Polymerization::parse_string(file_content)?;

    println!("part 1: {}", input.polymerize(10));
    println!("part 2: {}", input.polymerize(40));

    Ok(())
}

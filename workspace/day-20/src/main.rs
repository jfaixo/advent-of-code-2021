use std::{env, fs};
use std::error::Error;
use std::process::exit;
use crate::models::Input;

mod models;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let file_content =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");
    let input = Input::parse_string(file_content)?;

    println!("part 1: {}", input.multiple_enhance(2));
    println!("part 2: {}", input.multiple_enhance(50));

    Ok(())
}

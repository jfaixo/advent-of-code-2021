use crate::models::TransparentSheet;
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
    let input = TransparentSheet::parse_string(file_content)?;

    println!("part 1: {}", input.count_fold_once());
    input.fold_and_print();

    Ok(())
}

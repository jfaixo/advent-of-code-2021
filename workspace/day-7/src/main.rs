use crate::input_parsing::parse_file;
use std::env;
use std::error::Error;
use std::process::exit;

mod input_parsing;
mod models;
mod part_1;
mod part_2;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let input = parse_file(args[1].clone())?;

    println!("part 1: {}", part_1::cheapest_alignment(&input));
    println!("part 2: {}", part_2::cheapest_alignment(&input));

    Ok(())
}

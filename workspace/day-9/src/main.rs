use std::env;
use std::error::Error;
use std::process::exit;
use crate::input_parsing::parse_file;

mod models;
mod input_parsing;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let input  = parse_file(args[1].clone())?;

    println!("part 1: {}", input.map.risk_level());
    println!("part 2: {}", input.map.find_basins());

    Ok(())
}

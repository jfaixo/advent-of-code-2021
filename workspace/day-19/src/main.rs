use std::{env, fs};
use std::error::Error;
use std::process::exit;
use crate::models::Scanners;

mod models;
mod consts;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let file_content =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");
    let input = Scanners::parse_string(file_content)?;

    input.solve();

    Ok(())
}

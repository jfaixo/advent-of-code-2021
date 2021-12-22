use std::{env, fs};
use std::process::exit;
use crate::models::BootProcedure;

mod models;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let file_content =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");
    let input = BootProcedure::parse_string(file_content);

    println!("part 1: {}", input.part_1_count_on());
    println!("part 2: {}", input.part_2_count_on());
}

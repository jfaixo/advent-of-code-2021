mod input_parsing;
mod models;
mod part_1;
mod part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let input  = parse_file(args[1].clone());

    println!("part 1: {}", part_1::diagnostic_report(&input));
    println!("part 2: {}", part_2::diagnostic_report(&input));
}

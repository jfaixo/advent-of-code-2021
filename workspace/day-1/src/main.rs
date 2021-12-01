use std::env;
use std::process::exit;
use crate::input_parsing::parse_file;
use crate::part_1::depth_increase_count;
use crate::part_2::sliding_window_depth_increase_count;

mod input_parsing;
mod models;
mod part_1;
mod part_2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let input  = parse_file(args[1].clone());

    println!("part 1: {}", depth_increase_count(&input.data));
    println!("part 1: {}", sliding_window_depth_increase_count(&input));
}

use std::error::Error;
use std::process::exit;
use std::{env, fs};
use crate::models::BITSPacket;

mod models;
mod part_1;
mod part_2;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let file_content =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");
    let input = BITSPacket::parse_string(file_content)?;

    println!("part 1: {}", part_1::sum_header_versions(&input));
    println!("part 2: {}", part_2::compute_packet_value(&input));

    Ok(())
}

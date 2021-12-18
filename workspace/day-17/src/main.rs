use std::error::Error;
use std::process::exit;
use std::{env, fs};
use crate::models::TargetArea;

mod models;

fn main() -> Result<(), Box<dyn Error>> {
    let input = TargetArea {
        x_range: [111, 161],
        y_range: [-154, -101]
    };

    println!("part 1: {}", input.find_highest_hit());
    println!("part 2: {}", input.count_viable_initial_velocity());

    Ok(())
}

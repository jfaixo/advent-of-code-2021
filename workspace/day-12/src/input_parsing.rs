use crate::models::CaveGraph;
use std::error::Error;
use std::fs;

pub fn parse_file(file_path: String) -> Result<CaveGraph, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path).expect("Error while reading the data file");

    CaveGraph::parse_string(file_content)
}

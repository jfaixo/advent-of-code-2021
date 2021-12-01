use crate::depth_increase_count;
use crate::models::Input;

pub fn sliding_window_depth_increase_count(input: &Input) -> u32 {
    // Input is not that long, just transform our raw data to an aggregated data with a petty ugly
    // map
    let aggregated_data = input.data[2..]
        .iter()
        .enumerate()
        .map(|(i, &depth)| {
            input.data[2 + i - 2] + input.data[2 + i - 1] + depth
        }).collect();

    // Reuse the depth increase count from part 1
    depth_increase_count(&aggregated_data)
}

pub fn depth_increase_count(data: &Vec<i32>) -> u32 {
    let mut count = 0;

    let mut previous_deth = data[0];

    data[1..].iter().for_each(|&next_depth| {
        if next_depth > previous_deth {
            count += 1;
        }
        previous_deth = next_depth;
    });

    count
}

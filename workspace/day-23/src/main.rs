use crate::models_part_1::AmphipodState::NotMoved;

mod models_part_1;
mod consts;
mod models_part_2;

fn main() {
    let initial_state = models_part_1::GameState {
        amphipods_position_a: (1 << 5) | (1 << 8),
        amphipods_position_b: (1 << 2) | (1 << 9),
        amphipods_position_c: (1 << 6) | (1 << 12),
        amphipods_position_d: (1 << 3) | (1 << 11),
        amphipod_states: [
            NotMoved, NotMoved,
            NotMoved, NotMoved,
            NotMoved, NotMoved,
            NotMoved, NotMoved,
        ],
        energy_used: 0,
    };

    let mut optimizer = models_part_1::Optimizer::new();

    println!("part1: {}", optimizer.optimize(initial_state));


    let initial_state = models_part_2::GameState {
        amphipods_position_a: [(1 << 7), (1 << 12), (1 << 14), (1 << 18)],
        amphipods_position_b: [(1 << 2), (1 << 9), (1 << 13), (1 << 15)],
        amphipods_position_c: [(1 << 8), (1 << 10), (1 << 19), (1 << 20)],
        amphipods_position_d: [(1 << 3), (1 << 4), (1 << 5), (1 << 17)],
        amphipod_states: [
            NotMoved, NotMoved, NotMoved, NotMoved,
            NotMoved, NotMoved, NotMoved, NotMoved,
            NotMoved, NotMoved, NotMoved, NotMoved,
            NotMoved, NotMoved, NotMoved, NotMoved,
        ],
        energy_used: 0,
    };

    let mut optimizer = models_part_2::Optimizer::new();

    println!("part2: {}", optimizer.optimize(initial_state));
}

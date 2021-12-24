use std::collections::BinaryHeap;
use crate::consts::{DISTANCES_2};
use crate::models_part_1::AmphipodState;
use crate::models_part_1::AmphipodState::{FirstMoved, LastMoved, NotMoved};

#[derive(Debug, Copy, Clone)]
pub struct GameState {
    /// In order, positions of A1, A2, B1, B2, C1, C2, D1, D2
    ///
    /// ---------------------------------
    /// | 0  1     6    11    16    21 22|
    ///       | 2|  | 7|  |12|  |17|
    ///       | 3|  | 8|  |13|  |18|
    ///       | 4|  | 9|  |14|  |19|
    ///       | 5|  |10|  |15|  |20|
    ///
    /// Positions are bitboarded in order to simplify some possible moves computations
    pub amphipods_position_a: [u32; 4],
    pub amphipods_position_b: [u32; 4],
    pub amphipods_position_c: [u32; 4],
    pub amphipods_position_d: [u32; 4],
    pub amphipod_states : [AmphipodState; 16],
    pub energy_used: usize,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            amphipods_position_a: [0, 0, 0, 0],
            amphipods_position_b: [0, 0, 0, 0],
            amphipods_position_c: [0, 0, 0, 0],
            amphipods_position_d: [0, 0, 0, 0],
            amphipod_states: [NotMoved; 16],
            energy_used: 0
        }
    }
}

const BIT_MASK_HALLWAY :u32 = 0b000000000_11_0000_1_0000_1_0000_1_0000_11;
const BIT_MASK_AMBER_BOX :u32 = 0b000000000_00_0000_0_0000_0_0000_0_1111_00;
const BIT_MASK_BRONZE_BOX :u32 = 0b000000000_00_0000_0_0000_0_1111_0_0000_00;
const BIT_MASK_COPPER_BOX :u32 = 0b000000000_00_0000_0_1111_0_0000_0_0000_00;
const BIT_MASK_DESERT_BOX :u32 = 0b000000000_00_1111_0_0000_0_0000_0_0000_00;

fn get_amphipod_box_mask(amphipod_index: usize) -> u32 {
    match amphipod_index {
        0  | 1  | 2  | 3  => BIT_MASK_AMBER_BOX,
        4  | 5  | 6  | 7  => BIT_MASK_BRONZE_BOX,
        8  | 9  | 10 | 11 => BIT_MASK_COPPER_BOX,
        12 | 13 | 14 | 15 => BIT_MASK_DESERT_BOX,
        _ => panic!()
    }
}

fn is_stuck(all_amphipods: u32, current_position: u32) -> bool {
    if current_position == 3 || current_position == 8 || current_position == 13 || current_position == 18 {
        all_amphipods & (1 << current_position - 1) != 0
    }
    else if current_position == 4 || current_position == 9 || current_position == 14 || current_position == 19 {
        all_amphipods & (1 << current_position - 1) != 0 || all_amphipods & (1 << current_position - 2) != 0
    }
    else if current_position == 5 || current_position == 10 || current_position == 15 || current_position == 20 {
        all_amphipods & (1 << current_position - 1) != 0 || all_amphipods & (1 << current_position - 2) != 0 || all_amphipods & (1 << current_position - 3) != 0
    }
    else {
        false
    }
}

impl GameState {
    fn print_debug(&self) {
        for i in 0..16 {
            eprint!("{}={}, ", i, self.current_position(i));
        }
        eprintln!("energy={}", self.energy_used);
    }

    fn amphipod_positions(&self, amphipod_index: usize) -> u32 {
        match amphipod_index {
            0  | 1  | 2  | 3  => self.amphipods_position_a[0] | self.amphipods_position_a[1] | self.amphipods_position_a[2] | self.amphipods_position_a[3],
            4  | 5  | 6  | 7  => self.amphipods_position_b[0] | self.amphipods_position_b[1] | self.amphipods_position_b[2] | self.amphipods_position_b[3],
            8  | 9  | 10 | 11 => self.amphipods_position_c[0] | self.amphipods_position_c[1] | self.amphipods_position_c[2] | self.amphipods_position_c[3],
            12 | 13 | 14 | 15 => self.amphipods_position_d[0] | self.amphipods_position_d[1] | self.amphipods_position_d[2] | self.amphipods_position_d[3],
            _ => panic!()
        }
    }

    fn current_position(&self, amphipod_index: usize) -> u32 {
        let mask = match amphipod_index {
            0 => self.amphipods_position_a[0],
            1 => self.amphipods_position_a[1],
            2 => self.amphipods_position_a[2],
            3 => self.amphipods_position_a[3],
            4 => self.amphipods_position_b[0],
            5 => self.amphipods_position_b[1],
            6 => self.amphipods_position_b[2],
            7 => self.amphipods_position_b[3],
            8 => self.amphipods_position_c[0],
            9 => self.amphipods_position_c[1],
            10 => self.amphipods_position_c[2],
            11 => self.amphipods_position_c[3],
            12 => self.amphipods_position_d[0],
            13 => self.amphipods_position_d[1],
            14 => self.amphipods_position_d[2],
            _ => self.amphipods_position_d[3],
        };

        mask.trailing_zeros()
    }

    fn update_position(&mut self, amphipod_index: usize, value: u32) {
        match amphipod_index {
            0 => self.amphipods_position_a[0]  = value,
            1 => self.amphipods_position_a[1]  = value,
            2 => self.amphipods_position_a[2]  = value,
            3 => self.amphipods_position_a[3]  = value,
            4 => self.amphipods_position_b[0]  = value,
            5 => self.amphipods_position_b[1]  = value,
            6 => self.amphipods_position_b[2]  = value,
            7 => self.amphipods_position_b[3]  = value,
            8 => self.amphipods_position_c[0]  = value,
            9 => self.amphipods_position_c[1]  = value,
            10 => self.amphipods_position_c[2] = value,
            11 => self.amphipods_position_c[3] = value,
            12 => self.amphipods_position_d[0] = value,
            13 => self.amphipods_position_d[1] = value,
            14 => self.amphipods_position_d[2] = value,
            _ => self.amphipods_position_d[3]  = value,
        };
    }

    fn amphipod_possible_moves(&self, amphipod_index: usize) -> u32 {
        let box_mask = get_amphipod_box_mask(amphipod_index);

        match self.amphipod_states[amphipod_index] {
            AmphipodState::NotMoved | AmphipodState::FirstMoved => {
                let current_position = self.current_position(amphipod_index);
                let all_amphipods = self.amphipod_positions(0) | self.amphipod_positions(4) | self.amphipod_positions(8) | self.amphipod_positions(12);
                // First, check if we are stuck
                if is_stuck(all_amphipods, current_position) {
                    return 0;
                }

                // First possible moves are all the hallway + the amphipod final destination
                let mut possible_moves = BIT_MASK_HALLWAY | box_mask;

                // Remove moves on positions where there are other amphipods
                possible_moves &= !all_amphipods;

                // Filter impossible moves because blocked

                let mut not_possible = false;
                for i in (0..current_position).rev() {
                    if not_possible {
                        possible_moves &= !(1 << i);
                    }
                    else if is_box(i) {

                    }
                    else if possible_moves & (1 << i) == 0 {
                        not_possible = true;
                    }
                }

                let mut not_possible = false;
                for i in current_position + 1..=22 {
                    if not_possible {
                        possible_moves &= !(1 << i);
                    }
                    else if is_box(i) {

                    }
                    else if possible_moves & (1 << i) == 0 {
                        not_possible = true;
                    }
                }

                // If there is an other kind of amphipod in its box, do not allow entrance to the box
                let other_amphipods = match amphipod_index {
                    0  | 1  | 2  | 3  => self.amphipod_positions(4) | self.amphipod_positions(8) | self.amphipod_positions(12),
                    4  | 5  | 6  | 7  => self.amphipod_positions(0) | self.amphipod_positions(8) | self.amphipod_positions(12),
                    8  | 9  | 10 | 11 => self.amphipod_positions(0) | self.amphipod_positions(4) | self.amphipod_positions(12),
                    12 | 13 | 14 | 15 => self.amphipod_positions(0) | self.amphipod_positions(4) | self.amphipod_positions(8),
                    _ => panic!()
                };
                if other_amphipods & box_mask != 0 {
                    possible_moves &= !box_mask;
                }
                else {
                    // Keep only the move that goes deepest in the box
                    let offset = match amphipod_index {
                        0  | 1  | 2  | 3  => 2,
                        4  | 5  | 6  | 7  => 7,
                        8  | 9  | 10 | 11 => 12,
                        12 | 13 | 14 | 15 => 17,
                        _ => panic!()
                    };

                    if all_amphipods & (1 << (offset + 3)) == 0 {
                        possible_moves &= !((1 << offset) | (1 << offset + 1) | (1 << offset + 2));
                    }
                    else if all_amphipods & (1 << (offset + 2)) == 0 {
                        possible_moves &= !((1 << offset) | (1 << offset + 1));
                    }
                    else if all_amphipods & (1 << (offset + 1)) == 0 {
                        possible_moves &= !(1 << offset);
                    }
                }

                // Finally, optimize: if the amphipod can go to its room, limit the move to that
                if possible_moves & box_mask != 0 {
                    possible_moves &= box_mask;
                }

                // Limit only to moves in its own box if first moved
                if self.amphipod_states[amphipod_index] == FirstMoved {
                    possible_moves &= box_mask;
                }

                possible_moves
            }
            AmphipodState::LastMoved => {
                // Do nothing, he is good where he is
                0
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.amphipod_positions(0) == BIT_MASK_AMBER_BOX &&
            self.amphipod_positions(4) == BIT_MASK_BRONZE_BOX &&
            self.amphipod_positions(8) == BIT_MASK_COPPER_BOX &&
            self.amphipod_positions(12) == BIT_MASK_DESERT_BOX
    }
}

fn energy_for_move(initial_position: u32, final_position: u32) -> usize {
    DISTANCES_2[initial_position as usize][final_position as usize]
}

fn is_box(position: u32) -> bool {
    (position >= 2 && position <= 5) || (position >= 7 && position <= 10) || (position >= 12 && position <= 15) || (position >= 17 && position <= 20)
}

#[derive(Default)]
pub struct Optimizer {
    nodes: Vec<GameState>,
}

impl Optimizer {

    pub fn new() -> Self {
        let mut nodes: Vec<GameState> = Vec::with_capacity(500_000);
        unsafe {
            nodes.set_len(500_000);
        }

        Optimizer {
            nodes
        }
    }

    pub fn optimize(&mut self, initial_state: GameState) -> usize {
        self.nodes[0] = initial_state;

        let mut best_score = usize::MAX;

        let mut current_node_index = 1;
        while current_node_index > 0 {
            current_node_index -= 1;
            let current_state = self.nodes[current_node_index];

            // Check stop condition
            if current_state.is_finished() {
                if current_state.energy_used < best_score {
                    best_score = current_state.energy_used;
                }
            }
            else {
                for i in 0..16 {
                    let current_position = current_state.current_position(i);
                    let mut possible_moves = current_state.amphipod_possible_moves(i);
                    while possible_moves != 0 {
                        let possible_move = possible_moves.trailing_zeros();
                        possible_moves &= !(1 << possible_move);

                        self.nodes[current_node_index] = current_state;
                        match i {
                            0 | 1 | 2 | 3 => {
                                self.nodes[current_node_index].energy_used += energy_for_move(current_position, possible_move);
                                if self.nodes[current_node_index].energy_used < best_score {
                                    self.nodes[current_node_index].update_position(i,1 << possible_move);
                                    if is_box(possible_move) {
                                        self.nodes[current_node_index].amphipod_states[i] = LastMoved;
                                    }
                                    else {
                                        self.nodes[current_node_index].amphipod_states[i] = FirstMoved;
                                    }
                                    current_node_index += 1;
                                }
                            }
                            4 | 5 | 6 | 7 => {
                                self.nodes[current_node_index].energy_used += 10 * energy_for_move(current_position, possible_move);
                                if self.nodes[current_node_index].energy_used < best_score {
                                    self.nodes[current_node_index].update_position(i,1 << possible_move);
                                    if is_box(possible_move) {
                                        self.nodes[current_node_index].amphipod_states[i] = LastMoved;
                                    }
                                    else {
                                        self.nodes[current_node_index].amphipod_states[i] = FirstMoved;
                                    }
                                    current_node_index += 1;
                                }
                            }
                            8 | 9 | 10 | 11 => {
                                self.nodes[current_node_index].energy_used += 100 * energy_for_move(current_position, possible_move);
                                if self.nodes[current_node_index].energy_used < best_score {
                                    self.nodes[current_node_index].update_position(i,1 << possible_move);
                                    if is_box(possible_move) {
                                        self.nodes[current_node_index].amphipod_states[i] = LastMoved;
                                    }
                                    else {
                                        self.nodes[current_node_index].amphipod_states[i] = FirstMoved;
                                    }
                                    current_node_index += 1;
                                }
                            }
                            12 | 13 | 14 | 15 => {
                                self.nodes[current_node_index].energy_used += 1000 * energy_for_move(current_position, possible_move);
                                if self.nodes[current_node_index].energy_used < best_score {
                                    self.nodes[current_node_index].update_position(i,1 << possible_move);
                                    if is_box(possible_move) {
                                        self.nodes[current_node_index].amphipod_states[i] = LastMoved;
                                    }
                                    else {
                                        self.nodes[current_node_index].amphipod_states[i] = FirstMoved;
                                    }
                                    current_node_index += 1;
                                }
                            }
                            _ => panic!()
                        }
                    }
                }
            }
        }

        best_score
    }
}




#[cfg(test)]
mod tests {
    use crate::models_part_2::AmphipodState::{FirstMoved, LastMoved, NotMoved};
    use crate::models_part_2::{BIT_MASK_HALLWAY, GameState, Optimizer};

    #[test]
    fn example_case() {
        // ---------------------------------
        // | 0  1     6    11    16    21 22|
        //       | 2|  | 7|  |12|  |17|
        //       | 3|  | 8|  |13|  |18|
        //       | 4|  | 9|  |14|  |19|
        //       | 5|  |10|  |15|  |20|
        let initial_state = GameState {
            amphipods_position_a: [(1 << 5), (1 << 14), (1 << 18), (1 << 20)],
            amphipods_position_b: [(1 << 2), (1 << 9), (1 << 12), (1 << 13)],
            amphipods_position_c: [(1 << 7), (1 << 8), (1 << 15), (1 << 19)],
            amphipods_position_d: [(1 << 3), (1 << 4), (1 << 10), (1 << 17)],
            amphipod_states: [
                LastMoved, NotMoved, NotMoved, NotMoved,
                NotMoved, NotMoved, NotMoved, NotMoved,
                NotMoved, NotMoved, LastMoved, NotMoved,
                NotMoved, NotMoved, NotMoved, NotMoved,
            ],
            energy_used: 0,
        };

        let mut optimizer = Optimizer::new();

        assert_eq!(optimizer.optimize(initial_state), 44169);
    }

    #[test]
    fn possible_moves_1() {
        // ---------------------------------
        // | 0  1     6    11    16    21 22|
        //       | 2|  | 7|  |12|  |17|
        //       | 3|  | 8|  |13|  |18|
        //       | 4|  | 9|  |14|  |19|
        //       | 5|  |10|  |15|  |20|
        let initial_state = GameState {
            amphipods_position_a: [(1 << 5), (1 << 14), (1 << 18), (1 << 20)],
            amphipods_position_b: [(1 << 2), (1 << 9), (1 << 12), (1 << 13)],
            amphipods_position_c: [(1 << 7), (1 << 8), (1 << 15), (1 << 19)],
            amphipods_position_d: [(1 << 3), (1 << 4), (1 << 10), (1 << 17)],
            amphipod_states: [
                LastMoved, NotMoved, NotMoved, NotMoved,
                NotMoved, NotMoved, NotMoved, NotMoved,
                NotMoved, NotMoved, LastMoved, NotMoved,
                NotMoved, NotMoved, NotMoved, NotMoved,
            ],
            energy_used: 0,
        };

        assert_eq!(initial_state.amphipod_possible_moves(0), 0);
        assert_eq!(initial_state.amphipod_possible_moves(1), 0);
        assert_eq!(initial_state.amphipod_possible_moves(2), 0);
        assert_eq!(initial_state.amphipod_possible_moves(3), 0);

        assert_eq!(initial_state.amphipod_possible_moves(4), 0b000000000_11_0000_1_0000_1_0000_1_0000_11);
        assert_eq!(initial_state.amphipod_possible_moves(5), 0);
        assert_eq!(initial_state.amphipod_possible_moves(6), 0b000000000_11_0000_1_0000_1_0000_1_0000_11);
        assert_eq!(initial_state.amphipod_possible_moves(7), 0);
    }

    #[test]
    fn possible_moves_2() {
        // ---------------------------------
        // | 0  1     6    11    16    21 22|
        //       | 2|  | 7|  |12|  |17|
        //       | 3|  | 8|  |13|  |18|
        //       | 4|  | 9|  |14|  |19|
        //       | 5|  |10|  |15|  |20|
        let initial_state = GameState {
            amphipods_position_a: [(1 << 0), (1 << 1), (1 << 5), (1 << 20)],
            amphipods_position_b: [(1 << 2), (1 << 9), (1 << 16), (1 << 21)],
            amphipods_position_c: [(1 << 7), (1 << 8), (1 << 15), (1 << 19)],
            amphipods_position_d: [(1 << 3), (1 << 4), (1 << 10), (1 << 22)],
            amphipod_states: [
                FirstMoved, FirstMoved, LastMoved, NotMoved,
                NotMoved, NotMoved, FirstMoved, FirstMoved,
                NotMoved, NotMoved, LastMoved, NotMoved,
                NotMoved, NotMoved, NotMoved, FirstMoved,
            ],
            energy_used: 0,
        };

        assert_eq!(initial_state.amphipod_possible_moves(0), 0);
        assert_eq!(initial_state.amphipod_possible_moves(1), 0);
        assert_eq!(initial_state.amphipod_possible_moves(2), 0);
        assert_eq!(initial_state.amphipod_possible_moves(3), 0);

        assert_eq!(initial_state.amphipod_possible_moves(4), 0b000000000_00_0000_0_0000_1_0000_1_0000_00);
        assert_eq!(initial_state.amphipod_possible_moves(5), 0);
        assert_eq!(initial_state.amphipod_possible_moves(6), 0);
        assert_eq!(initial_state.amphipod_possible_moves(7), 0);

        assert_eq!(initial_state.amphipod_possible_moves(8), 0b000000000_00_0000_0_0100_0_0000_0_0000_00);
        assert_eq!(initial_state.amphipod_possible_moves(9), 0);
        assert_eq!(initial_state.amphipod_possible_moves(10), 0);
        assert_eq!(initial_state.amphipod_possible_moves(11), 0);
    }

    #[test]
    fn is_finished() {
        // ---------------------------------
        // | 0  1     6    11    16    21 22|
        //       | 2|  | 7|  |12|  |17|
        //       | 3|  | 8|  |13|  |18|
        //       | 4|  | 9|  |14|  |19|
        //       | 5|  |10|  |15|  |20|
        let initial_state = GameState {
            amphipods_position_a: [(1 << 2), (1 << 3), (1 << 4), (1 << 5)],
            amphipods_position_b: [(1 << 7), (1 << 8), (1 << 9), (1 << 10)],
            amphipods_position_c: [(1 << 12), (1 << 13), (1 << 14), (1 << 15)],
            amphipods_position_d: [(1 << 17), (1 << 18), (1 << 19), (1 << 20)],
            amphipod_states: [
                FirstMoved, FirstMoved, LastMoved, NotMoved,
                NotMoved, NotMoved, FirstMoved, FirstMoved,
                NotMoved, NotMoved, LastMoved, NotMoved,
                NotMoved, NotMoved, NotMoved, FirstMoved,
            ],
            energy_used: 0,
        };

        assert_eq!(initial_state.is_finished(), true);
    }
}

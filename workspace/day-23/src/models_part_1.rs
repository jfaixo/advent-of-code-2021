use crate::consts::DISTANCES;
use crate::models_part_1::AmphipodState::{FirstMoved, LastMoved, NotMoved};

#[derive(Debug, Copy, Clone)]
pub struct GameState {
    /// In order, positions of A1, A2, B1, B2, C1, C2, D1, D2
    ///
    /// ---------------------------------
    /// | 0  1     4     7    10    13 14|
    ///       | 2|  | 5|  | 8|  |11|
    ///       | 3|  | 6|  | 9|  |12|
    ///
    /// Positions are bitboarded in order to simplify some possible moves computations
    pub amphipods_position_a: u16,
    pub amphipods_position_b: u16,
    pub amphipods_position_c: u16,
    pub amphipods_position_d: u16,
    pub amphipod_states : [AmphipodState; 8],
    pub energy_used: usize,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            amphipods_position_a: 0,
            amphipods_position_b: 0,
            amphipods_position_c: 0,
            amphipods_position_d: 0,
            amphipod_states: [NotMoved; 8],
            energy_used: 0
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AmphipodState {
    NotMoved,
    FirstMoved,
    LastMoved,
}

const BIT_MASK_HALLWAY :u16 = 0b0110_0100_1001_0011;
const BIT_MASK_AMBER_BOX :u16 = 0b0000_0000_0000_1100;
const BIT_MASK_BRONZE_BOX :u16 = 0b0000_0000_0110_0000;
const BIT_MASK_COPPER_BOX :u16 = 0b0000_0011_0000_0000;
const BIT_MASK_DESERT_BOX :u16 = 0b0001_1000_0000_0000;

const BIT_MASK_AMBER_BOX_NEAR_HALLWAY :u16 = 0b0000_0000_0000_0100;
const BIT_MASK_BRONZE_BOX_NEAR_HALLWAY :u16 = 0b0000_0000_0010_0000;
const BIT_MASK_COPPER_BOX_NEAR_HALLWAY :u16 = 0b0000_0001_0000_0000;
const BIT_MASK_DESERT_BOX_NEAR_HALLWAY :u16 = 0b0000_1000_0000_0000;

fn get_first_amphipod_position(positions_mask: u16) -> u32 {
    positions_mask.trailing_zeros()
}

fn get_second_amphipod_position(positions_mask: u16) -> u32 {
    15 - positions_mask.leading_zeros()
}

fn get_amphipod_box_mask(amphipod_index: usize) -> u16 {
    match amphipod_index {
        0 | 1 => BIT_MASK_AMBER_BOX,
        2 | 3 => BIT_MASK_BRONZE_BOX,
        4 | 5 => BIT_MASK_COPPER_BOX,
        6 | 7 => BIT_MASK_DESERT_BOX,
        _ => panic!()
    }
}

impl GameState {
    fn print_debug(&self) {
        for i in 0..8 {
            eprint!("{}={}, ", i, self.current_position(i));
        }
        eprintln!("energy={}", self.energy_used);
    }

    fn amphipod_positions(&self, amphipod_index: usize) -> u16 {
        match amphipod_index {
            0 | 1 => self.amphipods_position_a,
            2 | 3 => self.amphipods_position_b,
            4 | 5 => self.amphipods_position_c,
            6 | 7 => self.amphipods_position_d,
            _ => panic!()
        }
    }

    fn current_position(&self, amphipod_index: usize) -> u32 {
        if amphipod_index % 2 == 0 {
            get_first_amphipod_position(self.amphipod_positions(amphipod_index))
        }
        else {
            get_second_amphipod_position(self.amphipod_positions(amphipod_index))
        }
    }

    fn amphipod_possible_moves(&self, amphipod_index: usize) -> u16 {
        let box_mask = get_amphipod_box_mask(amphipod_index);

        match self.amphipod_states[amphipod_index] {
            AmphipodState::NotMoved | AmphipodState::FirstMoved => {
                let current_position = self.current_position(amphipod_index);
                let all_amphipods = self.amphipods_position_a | self.amphipods_position_b | self.amphipods_position_c | self.amphipods_position_d;
                // First, check if we are stuck
                if current_position == 3 || current_position == 6 ||current_position == 9 || current_position == 12 {
                    if all_amphipods & (1 << current_position - 1) != 0 {
                        return 0;
                    }
                }

                // First possible moves are all the hallway + the amphipod final destination
                let mut possible_moves = BIT_MASK_HALLWAY | box_mask;

                // Remove moves on positions where there are other pods
                possible_moves &= !all_amphipods;

                // Filter impossible moves because blocked

                let mut not_possible = false;
                for i in (0..current_position).rev() {
                    if not_possible {
                        possible_moves &= !(1 << i);
                    }
                    else if i == 2 || i == 3 || i == 5 || i == 6 || i == 8 || i == 9 || i == 11 || i == 12 {

                    }
                    else if possible_moves & (1 << i) == 0 {
                        not_possible = true;
                    }
                }

                let mut not_possible = false;
                for i in current_position + 1..=14 {
                    if not_possible {
                        possible_moves &= !(1 << i);
                    }
                    else if i == 2 || i == 3 || i == 5 || i == 6 || i == 8 || i == 9 || i == 11 || i == 12 {

                    }
                    else if possible_moves & (1 << i) == 0 {
                        not_possible = true;
                    }
                }

                // If there is an other kind of amphipod in its box, do not allow entrance to the box
                let other_amphipods = match amphipod_index {
                    0 | 1 => self.amphipods_position_b | self.amphipods_position_c | self.amphipods_position_d,
                    2 | 3 => self.amphipods_position_a | self.amphipods_position_c | self.amphipods_position_d,
                    4 | 5 => self.amphipods_position_a | self.amphipods_position_b | self.amphipods_position_d,
                    6 | 7 => self.amphipods_position_a | self.amphipods_position_b | self.amphipods_position_c,
                    _ => panic!()
                };
                if other_amphipods & box_mask != 0 {
                    possible_moves &= !box_mask;
                }
                else if possible_moves & box_mask == box_mask {
                    // if the box is empty, only allow move to the end of the box
                    possible_moves &= match amphipod_index {
                        0 | 1 => !BIT_MASK_AMBER_BOX_NEAR_HALLWAY,
                        2 | 3 => !BIT_MASK_BRONZE_BOX_NEAR_HALLWAY,
                        4 | 5 => !BIT_MASK_COPPER_BOX_NEAR_HALLWAY,
                        6 | 7 => !BIT_MASK_DESERT_BOX_NEAR_HALLWAY,
                        _ => panic!()
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
        self.amphipod_states[0] == LastMoved && self.amphipod_states[1] == LastMoved && self.amphipods_position_a == BIT_MASK_AMBER_BOX &&
            self.amphipod_states[2] == LastMoved && self.amphipod_states[3] == LastMoved && self.amphipods_position_b == BIT_MASK_BRONZE_BOX &&
            self.amphipod_states[4] == LastMoved && self.amphipod_states[5] == LastMoved && self.amphipods_position_c == BIT_MASK_COPPER_BOX &&
            self.amphipod_states[6] == LastMoved && self.amphipod_states[7] == LastMoved && self.amphipods_position_d == BIT_MASK_DESERT_BOX
    }
}

fn energy_for_move(initial_position: u32, final_position: u32) -> usize {
    DISTANCES[initial_position as usize][final_position as usize]
}

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
                for i in 0..8 {
                    let current_position = current_state.current_position(i);
                    let mut possible_moves = current_state.amphipod_possible_moves(i);
                    while possible_moves != 0 {
                        let possible_move = possible_moves.trailing_zeros();
                        possible_moves &= !(1 << possible_move);

                        self.nodes[current_node_index] = current_state;
                        match i {
                            0 | 1 => {
                                self.nodes[current_node_index].energy_used += energy_for_move(current_position, possible_move);
                                if self.nodes[current_node_index].energy_used < best_score {
                                    self.nodes[current_node_index].amphipods_position_a =
                                        (current_state.amphipods_position_a & !(1 << current_position)) | (1 << possible_move);
                                    if possible_move == 2 || possible_move == 3 || possible_move == 5 || possible_move == 6 || possible_move == 8 || possible_move == 9 || possible_move == 11 || possible_move == 12 {
                                        self.nodes[current_node_index].amphipod_states[i] = LastMoved;
                                    }
                                    else {
                                        self.nodes[current_node_index].amphipod_states[i] = FirstMoved;
                                    }
                                    current_node_index += 1;
                                }
                            }
                            2 | 3 => {
                                self.nodes[current_node_index].energy_used += 10 * energy_for_move(current_position, possible_move);
                                if self.nodes[current_node_index].energy_used < best_score {
                                    self.nodes[current_node_index].amphipods_position_b =
                                        (current_state.amphipods_position_b & !(1 << current_position)) | (1 << possible_move);
                                    if possible_move == 2 || possible_move == 3 || possible_move == 5 || possible_move == 6 || possible_move == 8 || possible_move == 9 || possible_move == 11 || possible_move == 12 {
                                        self.nodes[current_node_index].amphipod_states[i] = LastMoved;
                                    }
                                    else {
                                        self.nodes[current_node_index].amphipod_states[i] = FirstMoved;
                                    }
                                    current_node_index += 1;
                                }
                            }
                            4 | 5 => {
                                self.nodes[current_node_index].energy_used += 100 * energy_for_move(current_position, possible_move);
                                if self.nodes[current_node_index].energy_used < best_score {
                                    self.nodes[current_node_index].amphipods_position_c =
                                        (current_state.amphipods_position_c & !(1 << current_position)) | (1 << possible_move);
                                    if possible_move == 2 || possible_move == 3 || possible_move == 5 || possible_move == 6 || possible_move == 8 || possible_move == 9 || possible_move == 11 || possible_move == 12 {
                                        self.nodes[current_node_index].amphipod_states[i] = LastMoved;
                                    }
                                    else {
                                        self.nodes[current_node_index].amphipod_states[i] = FirstMoved;
                                    }
                                    current_node_index += 1;
                                }
                            }
                            6 | 7 => {
                                self.nodes[current_node_index].energy_used += 1000 * energy_for_move(current_position, possible_move);
                                if self.nodes[current_node_index].energy_used < best_score {
                                    self.nodes[current_node_index].amphipods_position_d =
                                        (current_state.amphipods_position_d & !(1 << current_position)) | (1 << possible_move);
                                    if possible_move == 2 || possible_move == 3 || possible_move == 5 || possible_move == 6 || possible_move == 8 || possible_move == 9 || possible_move == 11 || possible_move == 12 {
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
    use crate::models_part_1::AmphipodState::{FirstMoved, LastMoved, NotMoved};
    use crate::models_part_1::{BIT_MASK_HALLWAY, GameState, Optimizer};

    #[test]
    fn example_case() {
        /// ---------------------------------
        /// | 0  1     4     7    10    13 14|
        ///       | 2|  | 5|  | 8|  |11|
        ///       | 3|  | 6|  | 9|  |12|
        let initial_state = GameState {
            amphipods_position_a: (1 << 3) | (1 << 12),
            amphipods_position_b: (1 << 2) | (1 << 8),
            amphipods_position_c: (1 << 5) | (1 << 9),
            amphipods_position_d: (1 << 6) | (1 << 11),
            amphipod_states: [
                LastMoved, NotMoved,
                NotMoved, NotMoved,
                NotMoved, LastMoved,
                NotMoved, NotMoved,
            ],
            energy_used: 0,
        };

        let mut optimizer = Optimizer::new();

        assert_eq!(optimizer.optimize(initial_state), 12521);
    }

    #[test]
    fn example_case_2() {
        let initial_state = GameState {
            amphipods_position_a: (1 << 3) | (1 << 12),
            amphipods_position_b: (1 << 5) | (1 << 6),
            amphipods_position_c: (1 << 8) | (1 << 9),
            amphipods_position_d: (1 << 7) | (1 << 11),
            amphipod_states: [
                LastMoved, NotMoved,
                LastMoved, LastMoved,
                LastMoved, LastMoved,
                FirstMoved, NotMoved,
            ],
            energy_used: 0,
        };

        let mut optimizer = Optimizer::new();

        assert_eq!(optimizer.optimize(initial_state), 9011);
    }

    #[test]
    fn possible_moves_1() {
        let initial_state = GameState {
            amphipods_position_a: (1 << 3) | (1 << 12),
            amphipods_position_b: (1 << 2) | (1 << 8),
            amphipods_position_c: (1 << 5) | (1 << 9),
            amphipods_position_d: (1 << 6) | (1 << 11),
            amphipod_states: [NotMoved; 8],
            energy_used: 0,
        };

        assert_eq!(initial_state.amphipod_possible_moves(0), 0);
        assert_eq!(initial_state.amphipod_possible_moves(1), 0);
        assert_eq!(initial_state.amphipod_possible_moves(2), BIT_MASK_HALLWAY);
        assert_eq!(initial_state.amphipod_possible_moves(3), BIT_MASK_HALLWAY);
        assert_eq!(initial_state.amphipod_possible_moves(4), BIT_MASK_HALLWAY);
        assert_eq!(initial_state.amphipod_possible_moves(5), 0);
        assert_eq!(initial_state.amphipod_possible_moves(6), 0);
        assert_eq!(initial_state.amphipod_possible_moves(7), BIT_MASK_HALLWAY);
    }

    #[test]
    fn possible_moves_2() {
        let initial_state = GameState {
            amphipods_position_a: (1 << 3) | (1 << 12),
            amphipods_position_b: (1 << 2) | (1 << 4),
            amphipods_position_c: (1 << 5) | (1 << 9),
            amphipods_position_d: (1 << 6) | (1 << 11),
            amphipod_states: [
                NotMoved, NotMoved,
                NotMoved, FirstMoved,
                NotMoved, LastMoved,
                NotMoved, NotMoved,
            ],
            energy_used: 0,
        };

        assert_eq!(initial_state.amphipod_possible_moves(0), 0);
        assert_eq!(initial_state.amphipod_possible_moves(1), 0);
        assert_eq!(initial_state.amphipod_possible_moves(2), 0b0000_0000_0000_0011);
        assert_eq!(initial_state.amphipod_possible_moves(3), 0);
        assert_eq!(initial_state.amphipod_possible_moves(4), 0b0000_0001_0000_0000);
        assert_eq!(initial_state.amphipod_possible_moves(5), 0);
        assert_eq!(initial_state.amphipod_possible_moves(6), 0);
        assert_eq!(initial_state.amphipod_possible_moves(7), 0b0110_0100_1000_0000);
    }

    #[test]
    fn possible_moves_3() {
        let initial_state = GameState {
            amphipods_position_a: (1 << 3) | (1 << 12),
            amphipods_position_b: (1 << 2) | (1 << 4),
            amphipods_position_c: (1 << 8) | (1 << 9),
            amphipods_position_d: (1 << 6) | (1 << 11),
            amphipod_states: [
                NotMoved, NotMoved,
                NotMoved, FirstMoved,
                LastMoved, LastMoved,
                NotMoved, NotMoved,
            ],
            energy_used: 0,
        };

        assert_eq!(initial_state.amphipod_possible_moves(0), 0);
        assert_eq!(initial_state.amphipod_possible_moves(1), 0);
        assert_eq!(initial_state.amphipod_possible_moves(2), 0b0000_0000_0000_0011);
        assert_eq!(initial_state.amphipod_possible_moves(3), 0);
        assert_eq!(initial_state.amphipod_possible_moves(4), 0);
        assert_eq!(initial_state.amphipod_possible_moves(5), 0);
        assert_eq!(initial_state.amphipod_possible_moves(6), 0b0110_0100_1000_0000);
        assert_eq!(initial_state.amphipod_possible_moves(7), 0b0110_0100_1000_0000);
    }

    #[test]
    fn possible_moves_4() {
        let initial_state = GameState {
            amphipods_position_a: (1 << 3) | (1 << 12),
            amphipods_position_b: (1 << 2) | (1 << 4),
            amphipods_position_c: (1 << 8) | (1 << 9),
            amphipods_position_d: (1 << 7) | (1 << 11),
            amphipod_states: [
                NotMoved, NotMoved,
                NotMoved, FirstMoved,
                LastMoved, LastMoved,
                FirstMoved, NotMoved,
            ],
            energy_used: 0,
        };

        assert_eq!(initial_state.amphipod_possible_moves(0), 0);
        assert_eq!(initial_state.amphipod_possible_moves(1), 0);
        assert_eq!(initial_state.amphipod_possible_moves(2), 0b0000_0000_0000_0011);
        assert_eq!(initial_state.amphipod_possible_moves(3), 0b0000_0000_0100_0000);
        assert_eq!(initial_state.amphipod_possible_moves(4), 0);
        assert_eq!(initial_state.amphipod_possible_moves(5), 0);
        assert_eq!(initial_state.amphipod_possible_moves(6), 0);
        assert_eq!(initial_state.amphipod_possible_moves(7), 0b0110_0100_0000_0000);
    }

    #[test]
    fn possible_moves_5() {
        let initial_state = GameState {
            amphipods_position_a: (1 << 3) | (1 << 13),
            amphipods_position_b: (1 << 5) | (1 << 6),
            amphipods_position_c: (1 << 8) | (1 << 9),
            amphipods_position_d: (1 << 7) | (1 << 10),
            amphipod_states: [
                LastMoved, FirstMoved,
                LastMoved, LastMoved,
                LastMoved, LastMoved,
                FirstMoved, FirstMoved,
            ],
            energy_used: 5513,
        };


        let mut optimizer = Optimizer::new();

        assert_eq!(optimizer.optimize(initial_state), 12521);
    }
}

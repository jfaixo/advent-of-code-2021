
#[derive(Debug, Clone, Copy)]
pub struct PawnPositions {
    player_1: u32,
    player_2: u32,
}

impl PawnPositions {
    pub fn new(player_1: u32, player_2: u32) -> Self {
        PawnPositions {
            player_1: player_1 - 1,
            player_2: player_2 - 1
        }
    }
}

#[derive(Debug, Default)]
struct DeterministicDie {
    current_value : u32,
    rolls: u32,
}

impl DeterministicDie {
    fn next_value(&mut self) -> u32 {
        self.rolls += 1;
        self.current_value += 1;

        if self.current_value > 100 {
            self.current_value = 1;
            return 1;
        }
        return self.current_value
    }
}

#[derive(Debug)]
pub struct Game {
    die: DeterministicDie,
    pawn_positions: PawnPositions,
    player_1_score: u32,
    player_2_score: u32,
    player_1_playing: bool
}

impl Game {
    pub fn new(starting_positions: PawnPositions) -> Self {
        Game {
            die: Default::default(),
            pawn_positions: starting_positions,
            player_1_score: 0,
            player_2_score: 0,
            player_1_playing: true
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct QuanticGameState {
    pawn_positions: PawnPositions,
    player_1_score: u32,
    player_2_score: u32,
    player_1_playing: bool
}

impl QuanticGameState {
    fn play_turn(&self, rolls: u32) -> Self {
        let mut new_state = *self;
        if new_state.player_1_playing {
            new_state.pawn_positions.player_1 = (new_state.pawn_positions.player_1 + rolls) % 10;
            new_state.player_1_score += new_state.pawn_positions.player_1 + 1;
        }
        else {
            new_state.pawn_positions.player_2 = (new_state.pawn_positions.player_2 + rolls) % 10;
            new_state.player_2_score += new_state.pawn_positions.player_2 + 1;
        }

        new_state.player_1_playing = !new_state.player_1_playing;

        new_state
    }
}

#[derive(Debug, Default)]
pub struct QuanticGame {
    player_1_wins: usize,
    player_2_wins: usize,
}

impl QuanticGame {
    pub fn play(&mut self, starting_positions: PawnPositions) -> usize {
        let state = QuanticGameState {
            pawn_positions: starting_positions,
            player_1_score: 0,
            player_2_score: 0,
            player_1_playing: true
        };

        self.play_turn(state, 1);

        return self.player_1_wins.max(self.player_2_wins);
    }

    pub fn play_turn(&mut self, state: QuanticGameState, game_count: usize) {
        if state.player_1_score >= 21 {
            self.player_1_wins += game_count;
        }
        else if state.player_2_score >= 21 {
            self.player_2_wins += game_count;
        }
        else {
            // Try all possible rollouts
            self.play_turn(state.play_turn(3), game_count * 1);
            self.play_turn(state.play_turn(4), game_count * 3);
            self.play_turn(state.play_turn(5), game_count * 6);
            self.play_turn(state.play_turn(6), game_count * 7);
            self.play_turn(state.play_turn(7), game_count * 6);
            self.play_turn(state.play_turn(8), game_count * 3);
            self.play_turn(state.play_turn(9), game_count * 1);
        }
    }
}

impl Game {
    fn play_turn(&mut self) {
        let rolls = self.die.next_value() + self.die.next_value() + self.die.next_value();

        if self.player_1_playing {
            self.pawn_positions.player_1 = (self.pawn_positions.player_1 + rolls) % 10;
            self.player_1_score += self.pawn_positions.player_1 + 1;
        }
        else {
            self.pawn_positions.player_2 = (self.pawn_positions.player_2 + rolls) % 10;
            self.player_2_score += self.pawn_positions.player_2 + 1;
        }

        self.player_1_playing = !self.player_1_playing;
    }

    pub fn play(&mut self) -> u32 {
        while self.player_1_score < 1000 && self.player_2_score < 1000 {
            self.play_turn();
        }

        if self.player_1_score >= 1000 {
            self.player_2_score * self.die.rolls
        }
        else {
            self.player_1_score * self.die.rolls
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{Game, QuanticGame};
    use crate::PawnPositions;

    #[test]
    fn example_case_play_turns() {
        let starting_positions = PawnPositions::new(4, 8);
        let mut game = Game::new(starting_positions);

        game.play_turn();
        assert_eq!(game.player_1_score, 10);
        assert_eq!(game.player_2_score, 0);

        game.play_turn();
        assert_eq!(game.player_1_score, 10);
        assert_eq!(game.player_2_score, 3);

        game.play_turn();
        assert_eq!(game.player_1_score, 14);
        assert_eq!(game.player_2_score, 3);

        game.play_turn();
        assert_eq!(game.player_1_score, 14);
        assert_eq!(game.player_2_score, 9);
    }

    #[test]
    fn example_case_play() {
        let starting_positions = PawnPositions::new(4, 8);
        let mut game = Game::new(starting_positions);

        assert_eq!(game.play(), 739785)
    }

    #[test]
    fn example_case_quantic_play() {
        let starting_positions = PawnPositions::new(4, 8);
        let mut game = QuanticGame::default();

        assert_eq!(game.play(starting_positions), 444356092776315);
    }
}

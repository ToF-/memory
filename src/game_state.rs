use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug,PartialEq)]
enum Status {
    LeftToPlay,
    RightToPlay,
    IllegalMove,
    LeftWins,
    RightWins
}

#[derive(Debug)]
struct Game {
    status: Status,
    tiles: Vec<u8>,
    undiscovered: u8,
    discovered: Vec<u8>,

}

impl Game {
    fn new() -> Game {
        let mut game = Game { 
            status: Status::LeftToPlay, 
            tiles: Vec::new(),
            undiscovered: 100,
            discovered: Vec::new(),
        };
        game.initialize();
        game
    }

    fn initialize(&mut self) {
        for row in 0..10 {
            for col in 0..10 {
                self.tiles.push((row % 5) * 10 + col);
                self.discovered.push(0)
            }
        }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        let mut values = self.tiles.as_mut_slice();
        values.shuffle(&mut rng);
        self.tiles = values.to_vec()
    }

    fn play(&mut self, row0: u8, col0: u8, row1: u8, col1: u8) {
        let undiscovered = self.undiscovered();
        if self.discovered[row0 as usize * 10 + col0 as usize] == 1
            || self.discovered[row1 as usize * 10 + col1 as usize] == 1 {
                self.status = Status::IllegalMove
            }
        else {
            if self.tile_at(row0, col0) == self.tile_at(row1, col1) {
                self.undiscovered -= 2;
                self.discovered[row0 as usize * 10 + col0 as usize] = 1;
                self.discovered[row1 as usize * 10 + col1 as usize] = 1;
            }
            if self.undiscovered() == undiscovered { 
                self.status = if self.status == Status::LeftToPlay { Status::RightToPlay } else { Status::LeftToPlay }
            }
            if self.undiscovered() == 0 {
                self.status = 
                if self.status == Status::LeftToPlay { Status::LeftWins } else { Status::RightWins }
            }
        }
    }

    fn tile_at(&self, row: u8, col: u8) -> u8 {
        self.tiles[row as usize * 10 + col as usize]
    }

    fn undiscovered(&self) -> u8 {
        100 - self.discovered.iter().sum::<u8>()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_initially_status_is_LeftToPlay() {
        let game: Game = Game::new();
        assert_eq!(Status::LeftToPlay, game.status);
    }

    #[test]
    fn test_after_left_move_status_is_RightToPlay() {
        let mut game: Game = Game::new();
        game.play(0,0,1,1);
        assert_eq!(Status::RightToPlay, game.status);
    }

    #[test]
    fn test_after_right_move_status_is_LeftToPlay() {
        let mut game: Game = Game::new();
        game.play(0,0,1,1);
        game.play(2,2,3,3);
        assert_eq!(Status::LeftToPlay, game.status);
    }

    #[test]
    fn test_when_not_shuffled_tiles_couples_are_ordered() {
        let game: Game = Game::new();
        assert_eq!(game.tile_at(0,0), 0);
        assert_eq!(game.tile_at(0,1), 1);
        assert_eq!(game.tile_at(1,0), 10);
        assert_eq!(game.tile_at(4,9), 49);
        assert_eq!(game.tile_at(5,0), 0);
        assert_eq!(game.tile_at(9,9), 49);
    }

    #[test]
    fn test_when_shuffled_tiles_couples_are_unordered() {
        let mut game: Game = Game::new();
        game.shuffle();
        let mut displaced_tiles: u8 = 0;
        for row in 0..10 {
            for col in 0..10 {
                let original = if row < 5 { row * 10 + col } else { (row - 5) * 10 + col };
                if game.tile_at(row, col) != original {
                    displaced_tiles += 1
                }
            }
        };
        assert!(displaced_tiles > 90);
    }

    #[test]
    fn test_initially_number_of_undiscovered_tiles_is_100() {
        let game: Game = Game::new();
        assert_eq!(100, game.undiscovered());
    }

    #[test]
    fn test_after_a_pair_is_found_number_of_undiscovered_is_decrease_by_two() {
        let mut game: Game = Game::new();
        game.play(1,1,2,2);
        assert_eq!(100, game.undiscovered());
        game.play(0,0,5,0);
        assert_eq!(98, game.undiscovered());
    }

    #[test]
    fn test_after_a_pair_is_found_the_tiles_cannot_be_played() {
        let mut game: Game = Game::new();
        game.play(0,0,5,0);
        assert_eq!(98, game.undiscovered());
        game.play(0,0,5,0);
        assert_eq!(Status::IllegalMove, game.status);
    }

    #[test]
    fn test_after_a_pair_is_found_the_same_player_gets_to_play() {
        let mut game: Game = Game::new();
        assert_eq!(Status::LeftToPlay, game.status);
        game.play(0,0,5,0);
        assert_eq!(Status::LeftToPlay, game.status);
    }

    #[test]
    fn test_after_all_tiles_are_discoverd_current_player_wins() {
        let mut game_1: Game = Game::new();
        for row in 0..5 {
            for col in 0..10 {
                game_1.play(row, col, row+5, col);
            }
        }
        assert_eq!(Status::LeftWins, game_1.status);
        let mut game_2: Game = Game::new();
        game_2.play(0,0,1,1);
        for row in 0..5 {
            for col in 0..10 {
                game_2.play(row, col, row+5, col);
            }
        }
        assert_eq!(Status::RightWins, game_2.status);
    }
}

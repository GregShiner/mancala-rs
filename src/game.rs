use std::fmt::{Display, Debug};

pub type PocketIndex = usize;
pub type PocketLocation = (PocketIndex, PlayerSide);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlayerSide {
    Player,
    Opponent
}

fn opposite_player(player_turn: PlayerSide) -> PlayerSide {
    // flip to opposite player
    match player_turn {
        PlayerSide::Player => PlayerSide::Opponent,
        PlayerSide::Opponent => PlayerSide::Player
    }
}

impl Display for PlayerSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerSide::Player => write!(f, "Player"),
            PlayerSide::Opponent => write!(f, "Opponent")
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    pub player_pockets: [i32; 7],
    pub opponent_pockets: [i32; 7],
    pub player_turn: PlayerSide // either Player or Opponent
}

impl Default for Board {
    fn default() -> Self {
        Board { 
            player_pockets: [4,4,4,4,4,4,0],
            opponent_pockets: [4,4,4,4,4,4,0],
            player_turn: PlayerSide::Player
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game { board: Board::default(), game_state: GameState::InProgress }
    }
}

impl Board {
    pub fn new(player_pockets: [i32; 7], opponent_pockets: [i32; 7], player_turn: PlayerSide) -> Self {
        Board {
            player_pockets: player_pockets,
            opponent_pockets: opponent_pockets,
            player_turn: player_turn
        }
    }
    fn switch_player(&mut self) {
        self.player_turn = opposite_player(self.player_turn)
    }

    pub fn get_stones(self, pocket:PocketLocation) -> i32 {
        match pocket.1 {
            PlayerSide::Player => self.player_pockets[pocket.0],
            PlayerSide::Opponent => self.opponent_pockets[pocket.0]
        }
    }

    fn pop_stones(&mut self, pocket:PocketLocation) -> i32 {
        let stones = self.get_stones(pocket);
        match pocket.1 {
            PlayerSide::Player => self.player_pockets[pocket.0] = 0,
            PlayerSide::Opponent => self.opponent_pockets[pocket.0] = 0
        }
        stones
    }

    fn increment_stones(&mut self, pocket:PocketLocation) {
        match pocket.1 {
            PlayerSide::Player => self.player_pockets[pocket.0] += 1,
            PlayerSide::Opponent => self.opponent_pockets[pocket.0] += 1
        }
    }

    fn pickup_stones(&mut self, pocket:PocketLocation) -> PocketLocation {
        let mut stones = self.pop_stones(pocket);
        let mut current_pocket = pocket.0;
        let mut side = pocket.1;
        while stones > 0 {
            current_pocket += 1;
            // if the player is dropping on their own side, increment their store when the end of the side (index 5) is reached, otherwise, skip the store
            if side == self.player_turn && current_pocket == 6 {
                self.increment_stones((6, side));
                stones -= 1;
                if stones == 0 {
                    break;
                }
                current_pocket = 0;
                side = opposite_player(side);
            } else if current_pocket == 7 || (side != self.player_turn && current_pocket == 6) {
                current_pocket = 0;
                side = opposite_player(side);
            }
            stones -= 1;
            self.increment_stones((current_pocket, side));
        }
        (current_pocket, side)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Game {
    pub board: Board,
    pub game_state: GameState
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let debug_game = DebugGame {
            board: self.board,
            game_state: Some(self.game_state),
            selected_pocket_location: None,
            stones: None
        };
        write!(f, "{:?}", debug_game)
    }
}

impl Game {
    pub fn new(board: Board) -> Self {
        Game { board: board, game_state: GameState::InProgress }
    }
    pub fn play_move(&mut self, pocket:PocketLocation) -> Result<(), InvalidPocketError> {
        /*
        A move consists of picking up the stones in a pocket and dropping them into the next pockets. 
        When the last stone is dropped, 3 things can happen
        1. If the last stone is dropped in the player's store, the player gets another turn
        2. If the last stone is dropped in a non-empty pocket, on either side, the player picks up the stones in that pocket and drops them again
        3. If the last stone is dropped in an empty pocket, the player's turn ends.
        */
        if pocket.1 != self.board.player_turn {
            return Err(InvalidPocketError::WrongPlayer);
        }
        if self.board.get_stones(pocket) == 0 {
            return Err(InvalidPocketError::EmptyPocket);
        }
        if pocket.0 == 6 {
            return Err(InvalidPocketError::StorePocket);
        }
        if !(0..6).contains(&pocket.0) {
            return Err(InvalidPocketError::OutOfBoundsPocket);
        }
        let (mut current_pocket, mut side) = self.board.pickup_stones(pocket);
        loop {
            if side == self.board.player_turn && current_pocket == 6 {
                break;
            } 
            if self.board.get_stones((current_pocket, side)) == 1 {
                self.board.switch_player();
                break;
            }
            (current_pocket, side) = self.board.pickup_stones((current_pocket, side));
        }
        // TODO: Reenable technical win check
        self.game_state = match self.check_for_game_end() {
            Some(winner) => GameState::Over(GameOver::Win(winner)),
            None => match self.check_for_technical_win() {
                Some(winner) => GameState::Over(GameOver::TechnicalWin(winner)),
                None => GameState::InProgress
            }
            // None => GameState::InProgress
        };
        Ok(())
    }

    fn check_for_game_end(&self) -> Option<Winner> {
        // if both sides still have some stones, the game is not over
        if self.board.player_pockets[0..6].iter().any(|&pocket| pocket != 0) && self.board.opponent_pockets[0..6].iter().any(|&pocket| pocket != 0) {
            return None;
        }
        // The winner is the player with the most stones in their store
        if self.board.player_pockets[6] > self.board.opponent_pockets[6] {
            Some(Winner::Player)
        } else if self.board.player_pockets[6] < self.board.opponent_pockets[6] {
            Some(Winner::Opponent)
        } else {
            Some(Winner::Tie)
        }
    }

    fn check_for_technical_win(&self) -> Option<PlayerSide> {
        // Checks if there are not enough stones left to change the current leader
        // get the remaining stones in the pockets (not including the stores)
        let remaining_stones: i32 = self.board.player_pockets[0..6].iter().sum::<i32>() + self.board.opponent_pockets[0..6].iter().sum::<i32>();
        let player_score = self.board.player_pockets[6];
        let opponent_score = self.board.opponent_pockets[6];
        // if the remaining stones plus the player's score is less than the opponent's score, the opponent wins
        if remaining_stones + player_score < opponent_score {
            return Some(PlayerSide::Opponent);
        } else if remaining_stones + opponent_score < player_score {
            return Some(PlayerSide::Player);
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    InProgress,
    Over(GameOver),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameOver {
    Win(Winner),
    TechnicalWin(PlayerSide),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Winner {
    Player,
    Opponent,
    Tie
}

#[derive(Debug)]
pub enum InvalidPocketError { // Error type for use in board.move
    EmptyPocket, // pocket is empty
    WrongPlayer, // pocket is on the wrong side
    StorePocket, // pocket is a store
    OutOfBoundsPocket // pocket is out of bounds
}

struct DebugGame {
    board: Board,
    selected_pocket_location: Option<PocketLocation>,
    stones: Option<i32>,
    game_state: Option<GameState>,
}

impl Debug for DebugGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn mark(pocket:PocketLocation, selected_pocket_location:Option<PocketLocation>) -> String {
            if let Some(selected_pocket_location) = selected_pocket_location {
                if selected_pocket_location == pocket {
                    return "->".to_string();
                }
            }
            "  ".to_string()
        }
        let opponent_store_str = format!("    {: >4}  {}", mark((6, PlayerSide::Opponent), self.selected_pocket_location), self.board.opponent_pockets[6]);
        let player_store_str = format!("    {: >4}  {}", mark((6, PlayerSide::Player), self.selected_pocket_location), self.board.player_pockets[6]);
        let opponent_pockets_str = (0..6).rev().map(|i| format!("{: >4}  {}", mark((i, PlayerSide::Opponent), self.selected_pocket_location), self.board.opponent_pockets[i])).collect::<Vec<String>>();
        let player_pockets_str = (0..6).map(|i| format!("{: >4}  {}", mark((i, PlayerSide::Player), self.selected_pocket_location), self.board.player_pockets[i])).collect::<Vec<String>>();
        let pocket_lines = (0..6).map(|i| format!("{}  {}", player_pockets_str[i], opponent_pockets_str[i])).collect::<Vec<String>>().join("\n");
        let stones_str = if let Some(stones) = self.stones {
            format!("Stones remaining: {}\n", stones)
        } else {
            "".to_string()
        };
        let state_str = if let Some(game_state) = &self.game_state {
            format!("Game state: {:?}\n", game_state)
        } else {
            "".to_string()
        };
        write!(f, "{}\n{}\n{}\n\n{}'s turn\n{}{}", opponent_store_str, pocket_lines, player_store_str, self.board.player_turn, stones_str, state_str)
    }
}

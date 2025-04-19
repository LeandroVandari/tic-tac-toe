/// Handles everything that has direct relation to the management of the game board.
/// Is driven by the [`Board`](board::Board) trait.
///
/// Contains the [`RecursiveBoard`](board::recursive::RecursiveBoard), which is the top level type
/// for this module.
pub mod board;

/// Represents the current state of a game.
/// 
/// How the board looks, which cell the next player has to move in, and which player's turn it is.
pub struct GameState {
    board: board::RecursiveBoard,
    // Is None if any (ongoing) cell can be chosen
    cell_to_play: Option<usize>,
    player_turn: crate::Player
}

impl GameState {
    /// Returns a new GameState, representing an empty and not-started game.
    #[must_use]
    pub fn new() -> Self {
        Self { board: board::RecursiveBoard::new(), cell_to_play: None, player_turn: crate::Player::Circle }
    }
}
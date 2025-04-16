pub mod board;

/// Represents the state of the winner of a board: either a player or a draw.
pub enum BoardWinner {
    Draw,
    Player(Player),
}

#[derive(PartialEq, Eq, Clone, Copy)]
/// Represents a player. Currently only circle and cross but maybe could have multiplayer later on.
pub enum Player {
    Circle,
    Cross,
}

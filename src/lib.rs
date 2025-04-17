pub mod board;

#[derive(PartialEq, Eq, Debug)]
/// Represents the result of a finished board: either a player has won or it's a draw.
/// If you want to represent a possibly on-going game, check [`BoardState`].
pub enum BoardResult {
    Draw,
    Winner(Player),
}

#[derive(Debug, PartialEq, Eq)]
/// Represents the state of a board. Either the game is in progress, or it's over and a [`BoardResult`] is available,
/// detailing the winner (if any).
pub enum BoardState {
    InProgress,
    Over(BoardResult),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
/// Represents a player. Currently only circle and cross but maybe could have multiplayer later on.
pub enum Player {
    Circle,
    Cross,
}

impl From<&Player> for char {
    fn from(value: &Player) -> Self {
        match value {
            Player::Circle => 'O',
            Player::Cross => 'X',
        }
    }
}

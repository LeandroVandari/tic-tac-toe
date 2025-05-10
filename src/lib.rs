#![warn(missing_docs)]
//! This crate is an implementation of a recursive Tic-Tac-Toe game, also known as the
//! "**Ultimate Tic-Tac-Toe**".

/// Deals with the state of the game, player moves, the board etc. It is the bare *Ultimate Tic-Tac-Toe* game/api.
pub mod game;

pub use game::board;

pub mod engine;

pub(crate) mod errors;

pub fn new() -> game::GameState {
    game::GameState::new()
}

#[derive(PartialEq, Eq, Debug, Clone)]
/// Represents the result of a finished board: either a player has won or it's a draw.
///
/// If you want to represent a possibly on-going game, check [`BoardState`].
pub enum BoardResult {
    /// A game that has had all cells filled without any of the players fullfilling the win conditions.
    Draw,
    /// A game that has ended because one of the [`Player`]s filled one of the win conditions. Contains said [`Player`].
    Winner(Player),
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// Represents the state of a board.
///
/// Either the game is in progress, or it's over and a [`BoardResult`] is available, detailing the winner (if any).
pub enum BoardState {
    /// A game that still hasn't finished: There are still empty cells and none of the [`Player`]s have fullfilled
    /// any of the win conditions.
    InProgress,
    /// A game that has finished, either in a draw or a [`Player`] has won. Check [`BoardResult`] for more information.
    Over(BoardResult),
}

impl BoardState {
    /// Returns whether the state is [`BoardState::InProgress`].    
    pub fn in_progress(&self) -> bool {
        matches!(self, BoardState::InProgress)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
/// Represents a player.
///
/// Currently only circle and cross but maybe could have multiplayer later on.
pub enum Player {
    /// The player represented by a circle (`O`).
    Circle,
    /// The player represented by a cross (`X`).
    Cross,
}

impl Player {
    /// Returns the next [`Player`] in the playing order.
    ///
    /// If the current player is a circle, the next is a cross, and vice-versa.
    ///
    /// # Examples:
    /// ```
    /// # use tic_tac_toe::Player;
    /// assert_eq!(Player::Circle.next(), Player::Cross);
    /// assert_eq!(Player::Cross.next(), Player::Circle);
    /// ```
    pub fn next(&self) -> Self {
        match self {
            Player::Circle => Player::Cross,
            Player::Cross => Player::Circle,
        }
    }
}

/// The [`Player`] should be representable by a single [`char`]`.
///
/// # Examples
/// ```
/// # use tic_tac_toe::Player;
/// assert_eq!(char::from(&Player::Circle), 'O');
/// assert_eq!(char::from(&Player::Cross), 'X');
/// ```
impl From<&Player> for char {
    fn from(value: &Player) -> Self {
        match value {
            Player::Circle => 'O',
            Player::Cross => 'X',
        }
    }
}

/// The [`Player`] has representable forms as [`char`]s.
///
/// # Examples
/// ```
/// # use tic_tac_toe::Player;
/// assert_eq!(Player::try_from('O'), Ok(Player::Circle));
/// assert_eq!(Player::try_from('X'), Ok(Player::Cross));
///
/// // Doesn't work with other chars:
/// assert!(Player::try_from('a').is_err());
/// assert!(Player::try_from('o').is_err());
/// assert!(Player::try_from('A').is_err());
/// ```
impl TryFrom<char> for Player {
    type Error = errors::InvalidPlayerChar;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Self::Circle),
            'X' => Ok(Self::Cross),
            _ => Err(errors::InvalidPlayerChar),
        }
    }
}

impl From<&BoardResult> for char {
    fn from(value: &BoardResult) -> Self {
        match value {
            BoardResult::Draw => '-',
            BoardResult::Winner(player) => player.into(),
        }
    }
}

impl From<&BoardState> for char {
    fn from(value: &BoardState) -> Self {
        match value {
            BoardState::InProgress => ' ',
            BoardState::Over(result) => result.into(),
        }
    }
}

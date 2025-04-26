use std::ops::{Deref, DerefMut};

use board::{Board, cell::Cell};

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
    player_turn: crate::Player,
}

impl GameState {
    /// Returns a new GameState, representing an empty and not-started game.
    #[must_use]
    pub fn new() -> Self {
        Self {
            board: board::RecursiveBoard::new(),
            cell_to_play: None,
            player_turn: crate::Player::Circle,
        }
    }

    /// Returns all of the available moves in a given position.
    pub fn available_moves(&self) -> AvailableMoves {
        if let Some(cell) = self.cell_to_play {
            let recursive_cell = &self.board[cell];
            assert!(
                recursive_cell.is_available(),
                "Cell that they can play in should be available."
            );

            recursive_cell
                .board()
                .available_cells()
                .map(|c| CellPosition::new(cell, c.0))
                .collect()
        } else {
            self.board
                .available_cells()
                .flat_map(|(idx, cell)| {
                    cell.board()
                        .available_cells()
                        .map(move |c| CellPosition::new(idx, c.0))
                })
                .collect()
        }
    }

    pub fn make_move(&mut self, position: CellPosition) -> Result<(), ()> {
        if !self.available_moves().contains(&position) {
            return Err(());
        }

        self.board.set_cell(&position, Some(self.player_turn));
        self.player_turn = self.player_turn.next();
        Ok(())
    }

    pub fn get_state(&self) -> crate::BoardState {
        self.board.get_state()
    }
}

/// All of the available moves in a given position.
pub struct AvailableMoves {
    available_moves: arrayvec::ArrayVec<CellPosition, 81>,
}

/// Represents a specific given inner cell in the [`RecursiveBoard`](board::RecursiveBoard).
#[derive(Debug, PartialEq, Eq)]
pub struct CellPosition {
    /// The index to the [`RecursiveCell`](board::recursive::RecursiveCell) directly contained by the [`RecursiveBoard`](board::RecursiveBoard).
    pub outer_cell: usize,
    /// The index to the inner player contained in the above mentioned [`RecursiveCell`](board::recursive::RecursiveCell).
    pub inner_cell: usize,
}

impl CellPosition {
    #[must_use]
    /// Returns a new [`Move`], with the provided cells.
    ///
    /// Checks for the validity of the cells (i.e. if they are in the board).
    pub fn new(outer_cell: usize, inner_cell: usize) -> Self {
        assert!(outer_cell < 9 && inner_cell < 9);
        Self {
            outer_cell,
            inner_cell,
        }
    }
}

impl FromIterator<CellPosition> for AvailableMoves {
    fn from_iter<T: IntoIterator<Item = CellPosition>>(iter: T) -> Self {
        Self {
            available_moves: iter.into_iter().collect(),
        }
    }
}

impl Deref for AvailableMoves {
    type Target = arrayvec::ArrayVec<CellPosition, 81>;
    fn deref(&self) -> &Self::Target {
        &self.available_moves
    }
}

impl DerefMut for AvailableMoves {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.available_moves
    }
}

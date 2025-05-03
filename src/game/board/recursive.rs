use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::{BoardResult, BoardState, Player, game::CellPosition};

use super::{Board, BoardDisplay, cell::Cell, inner::InnerBoard};
pub use cell::RecursiveCell;

/// A game board that contains game boards of itself. Each cell is stored as a [`RecursiveCell`], which
/// then contains the [`InnerBoard`] for that cell.
pub struct RecursiveBoard {
    cells: [RecursiveCell; 9],
}

impl RecursiveBoard {
    #[must_use]
    /// Returns a fresh [`RecursiveBoard`], with all cells empty.
    pub const fn new() -> Self {
        Self {
            cells: [const { RecursiveCell::new() }; 9],
        }
    }

    /// Sets the cell to its given owner.
    pub fn set_cell(&mut self, position: &CellPosition, owner: Option<Player>) {
        self[position.outer_cell].set_cell(position.inner_cell, owner);
    }
}

impl Board<RecursiveCell> for RecursiveBoard {
    fn cells<'a>(&'a self) -> impl Iterator<Item = &'a RecursiveCell>
    where
        RecursiveCell: 'a,
    {
        self.cells.iter()
    }
}

impl From<[InnerBoard; 9]> for RecursiveBoard {
    fn from(value: [InnerBoard; 9]) -> Self {
        Self {
            cells: value.map(RecursiveCell::from),
        }
    }
}

impl Default for RecursiveBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for RecursiveBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as BoardDisplay<_>>::fmt(self, f)
    }
}
impl Index<usize> for RecursiveBoard {
    type Output = RecursiveCell;
    fn index(&self, cell: usize) -> &Self::Output {
        debug_assert!(cell < 9);
        &self.cells[cell]
    }
}

impl IndexMut<usize> for RecursiveBoard {
    fn index_mut(&mut self, cell: usize) -> &mut Self::Output {
        debug_assert!(cell < 9);
        &mut self.cells[cell]
    }
}

impl Index<CellPosition> for RecursiveBoard {
    type Output = Option<Player>;
    fn index(&self, index: CellPosition) -> &Self::Output {
        &self[index.outer_cell].board()[index.inner_cell]
    }
}

/// Concerns the [`RecursiveCell`] type, which is in each cell of the [`RecursiveBoard`].
pub mod cell {
    use crate::Player;

    use super::*;

    #[derive(Debug, Clone)]
    /// The type that actually allows for us to have a [`RecursiveBoard`].
    ///
    /// Each [`RecursiveCell`] is made out of two components: The `board` and the `state`.
    ///
    /// The former contains the individual game itself, represented by an [`InnerBoard`],
    /// whilst the latter is basically a cache for the [`BoardState`] returned by the `board`'s
    /// [`Board::get_state`], so it doesn't need to be updated all the time.
    pub struct RecursiveCell {
        board: InnerBoard,
        state: BoardState,
    }

    impl RecursiveCell {
        #[must_use]
        /// Returns a [`RecursiveCell`] with a completely empty board.
        pub const fn new() -> Self {
            Self {
                board: InnerBoard::new(),
                state: BoardState::InProgress,
            }
        }

        /// Sets the value of the given `cell` in the contained [`InnerBoard`].
        pub fn set_cell(&mut self, cell: usize, owner: Option<Player>) {
            self.board[cell] = owner;
            self.state = self.board.get_state();
        }

        /// Returns the [`BoardState`] of the board contained by this cell.
        ///
        /// Is used for caching purposes.
        pub fn state(&self) -> &BoardState {
            &self.state
        }

        /// Returns the [`InnerBoard`] contained by this cell.
        pub fn board(&self) -> &InnerBoard {
            &self.board
        }
    }

    impl Cell for RecursiveCell {
        fn owner(&self) -> Option<&crate::Player> {
            match &self.state {
                BoardState::InProgress => None,
                BoardState::Over(result) => match result {
                    BoardResult::Draw => None,
                    BoardResult::Winner(player) => Some(player),
                },
            }
        }

        fn as_char(&self) -> char {
            char::from(self)
        }
    }

    impl From<InnerBoard> for RecursiveCell {
        fn from(value: InnerBoard) -> Self {
            Self {
                state: value.get_state(),
                board: value,
            }
        }
    }

    impl From<&RecursiveCell> for char {
        fn from(value: &RecursiveCell) -> Self {
            (&value.state).into()
        }
    }

    impl Default for RecursiveCell {
        fn default() -> Self {
            Self::new()
        }
    }
}

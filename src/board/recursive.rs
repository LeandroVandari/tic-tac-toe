use std::fmt::Display;

use crate::{BoardResult, BoardState};

use super::{Board, cell::Cell, inner::InnerBoard};
pub use cell::RecursiveCell;

pub struct RecursiveBoard {
    cells: [RecursiveCell; 9],
}

impl RecursiveBoard {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cells: [const { RecursiveCell::new() }; 9],
        }
    }
}

impl Board<RecursiveCell> for RecursiveBoard {
    fn get_cell(&self, cell: usize) -> &RecursiveCell {
        &self.cells[cell]
    }
}

impl From<[InnerBoard; 9]> for RecursiveBoard {
    fn from(value: [InnerBoard; 9]) -> Self {
        Self {
            cells: value.map(|board| RecursiveCell::from(board)),
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
        (self as &dyn Board<RecursiveCell>).fmt(f)
    }
}

pub mod cell {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct RecursiveCell {
        board: InnerBoard,
        pub(super) state: BoardState,
    }

    impl RecursiveCell {
        #[must_use]
        pub const fn new() -> Self {
            Self {
                board: InnerBoard::new(),
                state: BoardState::InProgress,
            }
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

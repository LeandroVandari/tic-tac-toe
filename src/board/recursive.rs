use crate::{BoardResult, BoardState};

use super::{inner::InnerBoard, Board};

pub struct RecursiveBoard {
    cells: [cell::RecursiveCell; 9]
}


impl RecursiveBoard {
    #[must_use]
    pub const fn new() -> Self {
        Self { cells: [const {cell::RecursiveCell::new()}; 9] }
    }
}

impl Board for RecursiveBoard {
    fn get_cell_owner(&self, cell: usize) -> Option<&crate::Player> {
        let cell = &self.cells[cell];

        match &cell.state {
            BoardState::InProgress => None,
            BoardState::Over(result) => match result {
                BoardResult::Draw => None,
                BoardResult::Winner(player) => Some(player)
            }
        }
    }
}


impl Default for RecursiveBoard {
    fn default() -> Self {
        Self::new()
    }
}

mod cell {
    use super::*;
    pub(super) struct RecursiveCell {
        board: InnerBoard,
        pub(super) state: BoardState
    }
    
    impl RecursiveCell {
        #[must_use]
        pub const fn new() -> Self {
            Self { board: InnerBoard::new(), state: BoardState::InProgress }
        }
    }
}
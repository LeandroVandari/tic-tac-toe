use std::fmt::Display;


use crate::{BoardResult, BoardState};

use super::{Board, inner::InnerBoard, cell::Cell};

pub struct RecursiveBoard {
    cells: [cell::RecursiveCell; 9],
}

impl RecursiveBoard {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cells: [const { cell::RecursiveCell::new() }; 9],
        }
    }
}

impl Board<cell::RecursiveCell> for RecursiveBoard {
    fn get_cell(&self, cell: usize) -> &cell::RecursiveCell {
        &self.cells[cell]
    }
}

impl From<[InnerBoard; 9]> for RecursiveBoard {
    fn from(value: [InnerBoard; 9]) -> Self {
        Self {
            cells: value.map(|board| cell::RecursiveCell::from(board)),
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
        const TEMPLATE_STR: &str = " 0 │ 1 │ 2 
———————————
 3 │ 4 │ 5 
———————————
 6 │ 7 │ 8 \
        ";

        let mut result_str = TEMPLATE_STR.to_string();
        for cell in 0..9 {
            result_str = result_str.replace(
                char::from_digit(cell, 10).unwrap(),
                char::from(&self.cells[cell as usize]).to_string().as_str(),
            )
        }

        write!(f, "{result_str}")
    }
}

mod cell {
    use super::*;

    #[derive(Debug, Clone)]
    pub(super) struct RecursiveCell {
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

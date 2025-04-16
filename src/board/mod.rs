#[cfg(test)]
mod tests;

use crate::{BoardState, Player, BoardResult};

/// The trait that represents a board. Allows to check for the states of cells, state of the board as a whole etc.
pub trait Board {
    /// Get the value of a single cell in the board, based on its index. Note that it has, effectively, two states:
    /// It can be **won** by a [`Player`], or not (in which case we return [`None`]). If it's not **won**, it might simply be empty,
    /// or a still contested board, in the case the type that implements this trait contains other [`Board`]s.
    fn get_cell(&self, cell: usize) -> Option<&Player>;

    /// Get the state of the board. It is either:
    ///  1. In progress, in which case we return [`None`].
    ///  2. Done, in which case we return the appropriate [`BoardWinner`], depending on the result.
    fn get_state(&self) -> BoardState {
        for group in 0..3 {
            // Rows
            if self.get_cell(group * 3).is_some() {
                let row_winner = self.get_cell(group * 3);
                let mut has_winner = true;

                for cell in 0..3 {
                    if self.get_cell(group * 3 + cell) != row_winner {
                        has_winner = false;
                        break;
                    }
                }
                if has_winner {
                    return BoardState::Over(BoardResult::Winner(*row_winner.unwrap()));
                }
            }

            // Cols
            if self.get_cell(group).is_some() {
                let col_winner = self.get_cell(group);
                let mut has_winner = true;

                for cell in 0..3 {
                    if self.get_cell(group + cell * 3) != col_winner {
                        has_winner = false;
                        break;
                    }
                }
                if has_winner {
                    return BoardState::Over(BoardResult::Winner(*col_winner.unwrap()));
                }
            }
        }

        // Diagonals: We use the fact that both diagonals intersect the center cell to just check if the extremities are equal to that.
        let center_cell = self.get_cell(4);
        if let Some(player) = center_cell {
            if (center_cell == self.get_cell(0) && center_cell == self.get_cell(8))
                || (center_cell == self.get_cell(2) && center_cell == self.get_cell(6))
            {
                return BoardState::Over(BoardResult::Winner(*player));
            }
        }

        // Check for a draw
        let mut is_draw = true;
        for cell in 0..9 {
            if self.get_cell(cell).is_none() {
                is_draw = false;
                break;
            }
        }
        if is_draw {
            return BoardState::Over(BoardResult::Draw);
        }

        BoardState::InProgress
    }
}

#[derive(PartialEq, Eq, Debug)]
/// The inner-most board in the game. All of its cells are either empty or belong to a player.
pub struct InnerBoard {
    cells: [Option<Player>; 9],
}

impl InnerBoard {
    #[must_use]
    /// Returns a new empty inner board.
    pub fn new() -> Self {
        Self {
            cells: [const { None }; 9],
        }
    }
}

impl Board for InnerBoard {
    fn get_cell(&self, cell: usize) -> Option<&Player> {
        self.cells[cell].as_ref()
    }
}

impl Default for InnerBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl From<[Option<Player>; 9]> for InnerBoard {
    fn from(value: [Option<Player>; 9]) -> Self {
        Self { cells: value }
    }
}

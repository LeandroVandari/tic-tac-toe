#[cfg(test)]
mod tests;

mod inner;
pub mod recursive;

use crate::{BoardResult, BoardState, Player};

/// The trait that represents a board. Allows to check for the states of cells, state of the board as a whole etc.
pub trait Board {
    /// Get the value of a single cell in the board, based on its index. Note that it has, effectively, two states:
    /// It can be **won** by a [`Player`], or not (in which case we return [`None`]). If it's not **won**, it might simply be empty,
    /// or a still contested board, in the case the type that implements this trait contains other [`Board`]s.
    ///
    /// # Panics
    /// This will panic if the requested `cell` is not inside the board.
    fn get_cell_owner(&self, cell: usize) -> Option<&Player>;

    /// Get the state of the game of the board. Check [`BoardState`] for information on the enum variants.
    fn get_state(&self) -> BoardState {
        for group in 0..3 {
            // Rows
            if self.get_cell_owner(group * 3).is_some() {
                let row_winner = self.get_cell_owner(group * 3);
                let mut has_winner = true;

                for cell in 0..3 {
                    if self.get_cell_owner(group * 3 + cell) != row_winner {
                        has_winner = false;
                        break;
                    }
                }
                if has_winner {
                    return BoardState::Over(BoardResult::Winner(*row_winner.unwrap()));
                }
            }

            // Cols
            if self.get_cell_owner(group).is_some() {
                let col_winner = self.get_cell_owner(group);
                let mut has_winner = true;

                for cell in 0..3 {
                    if self.get_cell_owner(group + cell * 3) != col_winner {
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
        let center_cell = self.get_cell_owner(4);
        if let Some(player) = center_cell {
            if (center_cell == self.get_cell_owner(0) && center_cell == self.get_cell_owner(8))
                || (center_cell == self.get_cell_owner(2) && center_cell == self.get_cell_owner(6))
            {
                return BoardState::Over(BoardResult::Winner(*player));
            }
        }

        // Check for a draw
        let mut is_draw = true;
        for cell in 0..9 {
            if self.get_cell_owner(cell).is_none() {
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

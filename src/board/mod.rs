#[cfg(test)]
mod tests;

mod cell;
mod inner;
pub mod recursive;

use std::fmt::Display;

use crate::{BoardResult, BoardState, Player};

/// The trait that represents a board. Allows to check for the states of cells, state of the board as a whole etc.
pub trait Board<T: cell::Cell> {
    /// Get the value of a single cell in the board, based on its index. Note that it has, effectively, two states:
    /// It can be **won** by a [`Player`], or not (in which case we return [`None`]). If it's not **won**, it might simply be empty,
    /// or a still contested board, in the case the type that implements this trait contains other [`Board`]s.
    ///
    /// # Panics
    /// This will panic if the requested `cell` is not inside the board.
    fn get_cell(&self, cell: usize) -> &T;

    /// Get the state of the game of the board. Check [`BoardState`] for information on the enum variants.
    fn get_state(&self) -> BoardState {
        for group in 0..3 {
            // Rows
            if self.get_cell(group * 3).owner().is_some() {
                let row_winner = self.get_cell(group * 3).owner();
                let mut has_winner = true;

                for cell in 0..3 {
                    if self.get_cell(group * 3 + cell).owner() != row_winner {
                        has_winner = false;
                        break;
                    }
                }
                if has_winner {
                    return BoardState::Over(BoardResult::Winner(*row_winner.unwrap()));
                }
            }

            // Cols
            if self.get_cell(group).owner().is_some() {
                let col_winner = self.get_cell(group).owner();
                let mut has_winner = true;

                for cell in 0..3 {
                    if self.get_cell(group + cell * 3).owner() != col_winner {
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
        let center_cell = self.get_cell(4).owner();
        if let Some(player) = center_cell {
            if (center_cell == self.get_cell(0).owner() && center_cell == self.get_cell(8).owner())
                || (center_cell == self.get_cell(2).owner()
                    && center_cell == self.get_cell(6).owner())
            {
                return BoardState::Over(BoardResult::Winner(*player));
            }
        }

        // Check for a draw
        let mut is_draw = true;
        for cell in 0..9 {
            if self.get_cell(cell).owner().is_none() {
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

impl<T: cell::Cell> Display for dyn Board<T> {
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
                self.get_cell(cell as usize).as_char().to_string().as_str(),
            );
        }

        write!(f, "{result_str}")
    }
}

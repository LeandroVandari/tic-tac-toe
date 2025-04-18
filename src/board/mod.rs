/// Deals with each individual cell. Is driven by the [`Cell`] trait.
///
/// More specifically, it concerns the abstractions required to be able
/// to generically deal with different types of cells.
pub mod cell;
/// Contains the [`InnerBoard`], and its required implementations.
pub mod inner;
/// Contains the [`RecursiveBoard`]: the driving type of this module, as it represents the board
/// of the Ultimate Tic-Tac-Toe game itself.
pub mod recursive;
#[cfg(test)]
mod tests;

use crate::{BoardResult, BoardState, Player};

/// The trait that represents a board. Allows to check for the states of cells, state of the board as a whole etc.
pub trait Board<T: cell::Cell> {
    /// Get the value of a single cell in the board, based on its index. The only requirement for the cell is that it implements
    /// [`Cell`](cell::Cell). That allows for the [`Cell::owner`](cell::Cell::owner) function to be called, which is all [`Board::get_state`] needs to know about.
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

/// A trait that implements a default [`fmt`](BoardDisplay::fmt) function that gives a reasonable
/// representation for all [`Board`]s.
///
/// It is blanket implemented on all [`Board`]s. However, implementing [`Display`](std::fmt::Display)
/// is still needed because, as it's a foreign trait, it cannot be implemented on generic types.
///
/// # Examples:
///
/// The recommended implementation of [`Display`](std::fmt::Display) is:
/// ```ignore
/// use tic_tac_toe::board::BoardDisplay;
/// impl std::fmt::Display for MyTypeThatImplementsBoard {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         <Self as BoardDisplay<_>>::fmt(self, f)
///     }
/// }
/// ```
pub trait BoardDisplay<T>: Board<T>
where
    T: cell::Cell,
{
    /// The method that allows for a general implementation of [`Display`](std::fmt::Display) for all implementers of [`Board`].
    ///
    /// Should be used as a simple redirection in the [`Display`](std::fmt::Display) implementation.
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

/// The blanket implementation of [`BoardDisplay`] that makes it available to all [`Board`]s.
impl<B, C> BoardDisplay<C> for B
where
    B: Board<C>,
    C: cell::Cell,
{
}

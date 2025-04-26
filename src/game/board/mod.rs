/// Deals with each individual cell. Is driven by the [`Cell`](cell::Cell) trait.
///
/// More specifically, it concerns the abstractions required to be able
/// to generically deal with different types of cells.
pub mod cell;
/// Contains the [`InnerBoard`](inner::InnerBoard), and its required implementations.
pub mod inner;

pub use inner::InnerBoard;

/// Contains the [`RecursiveBoard`]: the driving type of this module, as it represents the board

/// of the Ultimate Tic-Tac-Toe game itself.
pub mod recursive;
pub use recursive::RecursiveBoard;

#[cfg(test)]
mod tests;

use crate::{BoardResult, BoardState, Player};

/// The trait that represents a board. Allows to check for the states of cells, state of the board as a whole etc.
pub trait Board<T: cell::Cell>: std::ops::Index<usize, Output = T> {
    /// Return an iterator over the [`Cell`](cell::Cell)s of the board.
    fn cells<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a;

    /// Get the state of the game of the board. Check [`BoardState`] for information on the enum variants.
    ///
    /// # Examples
    /// ```
    /// # use std::str::FromStr;
    /// # use tic_tac_toe::*;
    /// use tic_tac_toe::board::InnerBoard;
    /// use tic_tac_toe::board::Board;
    ///
    /// let board = InnerBoard::from_str("OX-XXXO--").unwrap();
    ///
    /// assert_eq!(board.get_state(), BoardState::Over(BoardResult::Winner(Player::Cross)))
    /// ```
    fn get_state(&self) -> BoardState {
        for group in 0..3 {
            // Rows
            if self[group * 3].owner().is_some() {
                let row_winner = self[group * 3].owner();
                let mut has_winner = true;

                for cell in 0..3 {
                    if self[group * 3 + cell].owner() != row_winner {
                        has_winner = false;
                        break;
                    }
                }
                if has_winner {
                    return BoardState::Over(BoardResult::Winner(*row_winner.unwrap()));
                }
            }

            // Cols
            if self[group].owner().is_some() {
                let col_winner = self[group].owner();
                let mut has_winner = true;

                for cell in 0..3 {
                    if self[group + cell * 3].owner() != col_winner {
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
        let center_cell = self[4].owner();
        if let Some(player) = center_cell {
            if (center_cell == self[0].owner() && center_cell == self[8].owner())
                || (center_cell == self[2].owner() && center_cell == self[6].owner())
            {
                return BoardState::Over(BoardResult::Winner(*player));
            }
        }

        // Check for a draw
        let mut is_draw = true;
        for cell in 0..9 {
            if self[cell].owner().is_none() {
                is_draw = false;
                break;
            }
        }
        if is_draw {
            return BoardState::Over(BoardResult::Draw);
        }

        BoardState::InProgress
    }

    /// Returns all cells in the board that are available.
    ///
    /// Available cells are those where some player might still be able to make a move at some point in the future.
    fn available_cells<'a>(&'a self) -> impl Iterator<Item = (usize, &'a T)>
    where
        T: 'a,
    {
        self.cells()
            .enumerate()
            .filter(|cell| cell.1.is_available())
    }
}

/// A trait that implements a default [`fmt`](BoardDisplay::fmt) function that gives a reasonable
/// representation for all [`Board`]s.
///
/// It is blanket implemented on all [`Board`]s. However, implementing [`Display`](std::fmt::Display)
/// is still needed because, as it's a foreign trait, it cannot be implemented on generic types.
///
/// # Examples
/// ```
/// # use std::str::FromStr;
/// use tic_tac_toe::board::InnerBoard;
///
/// let board = InnerBoard::from_str("XXO--XOO-").unwrap();
/// ```
/// **Formatting that would look something like:**
/// ```text
///  X │ X │ O
/// ———————————
///    │   │ X
/// ———————————
///  O │ O │   
/// ```
///
/// # Implementing Display:
///
/// The recommended implementation of [`Display`](std::fmt::Display) is:
/// ```
/// # struct MyTypeThatImplementsBoard {c: C};
/// # struct C; // Some Cell
/// # impl tic_tac_toe::board::cell::Cell for C {fn owner(&self) -> Option<&tic_tac_toe::Player> {None} fn as_char(&self) -> char {'a'}}
/// # impl std::ops::Index<usize> for MyTypeThatImplementsBoard {type Output = C; fn index(&self, cell: usize) -> &Self::Output {&C}}
/// # impl tic_tac_toe::board::Board<C> for MyTypeThatImplementsBoard {fn cells<'a>(&'a self) -> impl Iterator<Item = &'a C> where C: 'a { std::iter::once(&self.c) }}
/// #
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
                self[cell as usize].as_char().to_string().as_str(),
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

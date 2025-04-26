use super::{Board, BoardDisplay, Player};
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(PartialEq, Eq, Debug, Clone)]
/// The inner-most board in the game. All of its cells are either empty or belong to a player.
pub struct InnerBoard {
    cells: [Option<Player>; 9],
}

impl InnerBoard {
    #[must_use]
    /// Returns a new empty inner board.
    ///
    /// # Examples
    /// ```
    /// use tic_tac_toe::board::InnerBoard;
    ///
    /// let board: InnerBoard = InnerBoard::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            cells: [const { None }; 9],
        }
    }

    /// Sets the given `cell`'s owner to the provided value.
    ///
    /// # Examples
    /// ```
    /// use tic_tac_toe::{Player, board::{InnerBoard, Board}};
    ///
    /// let mut board = InnerBoard::new();
    /// assert_eq!(board[0], None);
    ///
    /// board.set_cell(0, Some(Player::Cross));
    /// assert_eq!(board[0], Some(Player::Cross));
    ///
    /// // Other cells remain unchanged
    /// assert_eq!(board[1], None);
    /// ```
    pub fn set_cell(&mut self, cell: usize, owner: Option<Player>) {
        debug_assert!(cell < 9);
        self.cells[cell] = owner;
    }
}

impl Board<Option<Player>> for InnerBoard {
    fn cells<'a>(&'a self) -> impl Iterator<Item = &'a Option<Player>>
    where
        Option<Player>: 'a,
    {
        self.cells.iter()
    }
}

impl super::cell::Cell for Option<Player> {
    /// This is a no-op for this type.
    fn owner(&self) -> Option<&Player> {
        self.as_ref()
    }

    fn as_char(&self) -> char {
        if let Some(player) = self {
            player.into()
        } else {
            ' '
        }
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

impl Display for InnerBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as BoardDisplay<_>>::fmt(self, f)
    }
}

impl Index<usize> for InnerBoard {
    type Output = Option<Player>;
    fn index(&self, cell: usize) -> &Self::Output {
        debug_assert!(cell < 9);
        &self.cells[cell]
    }
}

impl IndexMut<usize> for InnerBoard {
    fn index_mut(&mut self, cell: usize) -> &mut Self::Output {
        debug_assert!(cell < 9);
        &mut self.cells[cell]
    }
}

impl FromStr for InnerBoard {
    type Err = crate::errors::InnerBoardFromStrError;
    /// Take the board as a single line string, with each cell represented by a single [`char`].
    /// Empty cells marked by `-`.
    ///
    /// ```
    /// # use tic_tac_toe::{Player, board::inner::InnerBoard};
    /// # use std::str::FromStr;
    /// let board = InnerBoard::from_str("OX-XXXO--").unwrap();
    /// assert_eq!(board, InnerBoard::from([Some(Player::Circle), Some(Player::Cross), None, Some(Player::Cross), Some(Player::Cross), Some(Player::Cross), Some(Player::Circle), None, None]))
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 9 {
            return Err(crate::errors::InnerBoardFromStrError::InvalidLength);
        }
        let mut board_array = [const { None }; 9];
        for (i, c) in s.chars().enumerate() {
            if c == '-' {
                continue;
            }
            board_array[i] = Some(Player::try_from(c)?);
        }

        Ok(InnerBoard::from(board_array))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_inner_board() {
        assert_eq!(
            InnerBoard::new(),
            InnerBoard {
                cells: [const { None }; 9]
            }
        )
    }
    #[test]
    fn display_inner_board() {
        let board = InnerBoard::from([
            Some(Player::Circle),
            Some(Player::Cross),
            None,
            Some(Player::Cross),
            Some(Player::Cross),
            Some(Player::Cross),
            Some(Player::Circle),
            None,
            None,
        ]);

        assert_eq!(
            format!("{board}"),
            " O │ X │   
———————————
 X │ X │ X 
———————————
 O │   │   \
        "
        );
    }
}

use super::{Board, Player};
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone)]
/// The inner-most board in the game. All of its cells are either empty or belong to a player.
pub struct InnerBoard {
    cells: [Option<Player>; 9],
}

impl InnerBoard {
    #[must_use]
    /// Returns a new empty inner board.
    pub const fn new() -> Self {
        Self {
            cells: [const { None }; 9],
        }
    }

    pub fn set_cell(&mut self, cell: usize, value: Option<Player>) {
        debug_assert!(cell < 9);
        self.cells[cell] = value;
    }
}

impl Board for InnerBoard {
    fn get_cell_owner(&self, cell: usize) -> Option<&Player> {
        debug_assert!(cell < 9);
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

impl Display for InnerBoard {
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
                (if let Some(player) = self.get_cell_owner(cell as usize) {
                    player.into()
                } else {
                    ' '
                })
                .to_string()
                .as_str(),
            );
        }

        write!(f, "{result_str}")
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

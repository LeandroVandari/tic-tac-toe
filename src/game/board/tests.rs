use crate::board::{cell::*, inner::*, *};

#[test]
fn get_cell() {
    let board = InnerBoard::from([
        None,
        None,
        Some(Player::Circle),
        None,
        Some(Player::Cross),
        None,
        None,
        None,
        None,
    ]);

    assert_eq!(board[0].owner(), None);
    assert_eq!(board[2].owner(), Some(&Player::Circle));
    assert_eq!(board[4].owner(), Some(&Player::Cross))
}

#[test]
fn get_board_state() {
    let board_empty = InnerBoard::new();
    assert_eq!(
        board_empty.get_state(),
        BoardState::InProgress,
        "Doesn't properly get empty board state."
    );

    let board_progress = InnerBoard::from([
        None,
        None,
        Some(Player::Circle),
        None,
        Some(Player::Cross),
        None,
        None,
        None,
        None,
    ]);
    assert_eq!(
        board_progress.get_state(),
        BoardState::InProgress,
        "Doesn't properly get in progress game state."
    );

    let board_draw = InnerBoard::from([
        Some(Player::Circle),
        Some(Player::Circle),
        Some(Player::Cross),
        Some(Player::Cross),
        Some(Player::Cross),
        Some(Player::Circle),
        Some(Player::Circle),
        Some(Player::Cross),
        Some(Player::Circle),
    ]);
    assert_eq!(
        board_draw.get_state(),
        BoardState::Over(BoardResult::Draw),
        "Doesn't recognize a draw."
    );

    let board_win_horizontal = InnerBoard::from([
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
        board_win_horizontal.get_state(),
        BoardState::Over(BoardResult::Winner(Player::Cross)),
        "Doesn't recognize horizontal win"
    );

    let board_win_vertical = InnerBoard::from([
        Some(Player::Circle),
        None,
        Some(Player::Cross),
        Some(Player::Circle),
        Some(Player::Cross),
        None,
        Some(Player::Circle),
        Some(Player::Cross),
        None,
    ]);
    assert_eq!(
        board_win_vertical.get_state(),
        BoardState::Over(BoardResult::Winner(Player::Circle)),
        "Doesn't recognize vertical win"
    );

    let board_win_diagonal = InnerBoard::from([
        Some(Player::Circle),
        None,
        Some(Player::Cross),
        None,
        Some(Player::Cross),
        None,
        Some(Player::Cross),
        Some(Player::Circle),
        Some(Player::Circle),
    ]);
    assert_eq!(
        board_win_diagonal.get_state(),
        BoardState::Over(BoardResult::Winner(Player::Cross)),
        "Doesn't recognize diagonal win"
    );
}

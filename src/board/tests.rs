use crate::board::*;

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
fn get_cell() {
    let board = InnerBoard {
        cells: [
            None,
            None,
            Some(Player::Circle),
            None,
            Some(Player::Cross),
            None,
            None,
            None,
            None,
        ],
    };

    assert_eq!(board.get_cell(0), None);
    assert_eq!(board.get_cell(2), Some(&Player::Circle));
    assert_eq!(board.get_cell(4), Some(&Player::Cross))
}

#[test]
fn get_board_state() {
    let board_empty = InnerBoard::new();
    assert_eq!(
        board_empty.get_state(),
        None,
        "Doesn't properly get empty board state."
    );

    let board_progress = InnerBoard {
        cells: [
            None,
            None,
            Some(Player::Circle),
            None,
            Some(Player::Cross),
            None,
            None,
            None,
            None,
        ],
    };
    assert_eq!(
        board_progress.get_state(),
        None,
        "Doesn't properly get in progress game state."
    );

    let board_draw = InnerBoard {
        cells: [
            Some(Player::Circle),
            Some(Player::Circle),
            Some(Player::Cross),
            Some(Player::Cross),
            Some(Player::Cross),
            Some(Player::Circle),
            Some(Player::Circle),
            Some(Player::Cross),
            Some(Player::Circle),
        ],
    };
    assert_eq!(
        board_draw.get_state(),
        Some(BoardWinner::Draw),
        "Doesn't recognize a draw."
    );

    let board_win_horizontal = InnerBoard {
        cells: [
            Some(Player::Circle),
            Some(Player::Cross),
            None,
            Some(Player::Cross),
            Some(Player::Cross),
            Some(Player::Cross),
            Some(Player::Circle),
            None,
            None,
        ],
    };
    assert_eq!(
        board_win_horizontal.get_state(),
        Some(BoardWinner::Player(Player::Cross)),
        "Doesn't recognize horizontal win"
    );

    let board_win_vertical = InnerBoard {
        cells: [
            Some(Player::Circle),
            None,
            Some(Player::Cross),
            Some(Player::Circle),
            Some(Player::Cross),
            None,
            Some(Player::Circle),
            Some(Player::Cross),
            None,
        ],
    };
    assert_eq!(
        board_win_vertical.get_state(),
        Some(BoardWinner::Player(Player::Circle)),
        "Doesn't recognize vertical win"
    );

    let board_win_diagonal = InnerBoard {
        cells: [
            Some(Player::Circle),
            None,
            Some(Player::Cross),
            None,
            Some(Player::Cross),
            None,
            Some(Player::Cross),
            Some(Player::Circle),
            Some(Player::Circle),
        ],
    };
    assert_eq!(
        board_win_diagonal.get_state(),
        Some(BoardWinner::Player(Player::Cross)),
        "Doesn't recognize diagonal win"
    );
}

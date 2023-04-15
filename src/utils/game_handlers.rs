use crate::Board;
pub fn handle_clicked_button(board: &mut Board, x: usize, y: usize) {
    if board.game_ended {
        return;
    };
    if board.grid[x][y] == "" {
        if board.current_player == 1 {
            board.grid[x][y] = "X";
            if check_winner(board) {
                return;
            };
            board.current_player = 2;
        } else {
            board.grid[x][y] = "O";
            if check_winner(board) {
                return;
            };
            board.current_player = 1;
        }
    }
}

pub fn check_winner(board: &mut Board) -> bool {
    let winner_positions = vec![
        vec![(0, 0), (0, 1), (0, 2)], // Check for horizontal wins in top row
        vec![(1, 0), (1, 1), (1, 2)], // Check for horizontal wins in middle row
        vec![(2, 0), (2, 1), (2, 2)], // Check for horizontal wins in bottom row
        vec![(0, 0), (1, 0), (2, 0)], // Check for vertical wins in left column
        vec![(0, 1), (1, 1), (2, 1)], // Check for vertical wins in middle column
        vec![(0, 2), (1, 2), (2, 2)], // Check for vertical wins in right column
        vec![(0, 0), (1, 1), (2, 2)], // Check for diagonal wins from top-left to bottom-right
        vec![(0, 2), (1, 1), (2, 0)], // Check for diagonal wins from top-right to bottom-left
    ];

    let mut winner = 0;

    for positions in winner_positions {
        for (x, y) in positions {
            let symbol = &board.grid[y][x];
            if symbol == &"X" {
                if winner == 0 {
                    winner = 1;
                } else if winner == 2 {
                    winner = 0;
                    break;
                }
            } else if symbol == &"O" {
                if winner == 0 {
                    winner = 2;
                } else if winner == 1 {
                    winner = 0;
                    break;
                }
            } else {
                winner = 0;
                break;
            }
        }
        if winner != 0 {
            board.winner = winner;
            board.game_ended = true;
            return true;
        }
    }

    return false;
}

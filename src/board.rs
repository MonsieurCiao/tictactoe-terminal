#[derive(Debug)]
pub struct Board {
    pub board: Vec<u8>,
    move_count: u8,
}
impl Board {
    pub fn new() -> Self {
        Self {
            board: vec![0; 9],
            move_count: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoardMove {
    pub idx: usize,
    pub symbol: char,
}

pub fn update_board(board: &mut Vec<u8>, board_move: BoardMove) {
    //convert symbol to number (1: player, 2: bot)
    let contestant = if board_move.symbol == 'x' { 1 } else { 2 };
    //update board
    board[board_move.idx] = contestant;
}
pub fn check_win(board_struct: &mut Board) -> Option<u8> {
    board_struct.move_count += 1;
    let board: &[u8] = &board_struct.board;
    let lines = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for line in lines {
        let [a, b, c] = line;
        if board[a] != 0 && board[a] == board[b] && board[b] == board[c] {
            return Some(board[a]);
        }
    }
    // no winner found, but if move count == 9, then end game
    if board_struct.move_count == 9 {
        Some(0)
    } else {
        None
    }
}

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

impl Clone for Board {
    fn clone(&self) -> Self {
        let mut board: Vec<u8> = vec![];
        for cell in &self.board {
            board.push(*cell);
        }
        Self {
            board,
            move_count: self.move_count, //here a copy happens!!
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BoardMove {
    pub idx: usize,
    pub symbol: char,
}

pub fn update_board(board_struct: &mut Board, board_move: BoardMove) {
    //convert symbol to number (1: player, 2: bot)
    let contestant = if board_move.symbol == 'x' { 1 } else { 2 };
    //update board
    board_struct.board[board_move.idx] = contestant;
    board_struct.move_count += 1;
}
pub fn check_win(board_struct: &Board) -> Option<u8> {
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

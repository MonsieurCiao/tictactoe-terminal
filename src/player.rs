use crate::BoardMove;
use crate::MoveError;
use crate::io;

pub fn player_move(board: &[u8]) -> Result<BoardMove, MoveError> {
    let mut buf = String::new();
    println!("Input row");
    io::stdin().read_line(&mut buf)?;
    let row: u8 = buf.trim().parse()?;
    if row < 1 || row > 3 {
        return Err(MoveError::InvalidRow);
    }

    println!("Input col");
    buf.clear();
    io::stdin().read_line(&mut buf)?;
    let col: u8 = buf.trim().parse()?;
    if col < 1 || col > 3 {
        return Err(MoveError::InvalidCol);
    }
    let board_idx = (row - 1) * 3 + col - 1;

    if board[(board_idx) as usize] != 0 {
        return Err(MoveError::AlreadyOccupied);
    }
    return Ok(BoardMove {
        idx: (board_idx) as usize,
        symbol: 'x',
    });
}

use rand::prelude::IndexedRandom;
use std::{io, panic};

#[derive(Debug, Clone, Copy)]
pub struct BoardMove {
    pub idx: usize,
    pub symbol: char,
}

#[derive(Debug)]
pub enum MoveError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
    InvalidRow,
    InvalidCol,
    AlreadyOccupied,
}

impl From<io::Error> for MoveError {
    fn from(e: io::Error) -> Self {
        MoveError::Io(e)
    }
}
impl From<std::num::ParseIntError> for MoveError {
    fn from(e: std::num::ParseIntError) -> Self {
        MoveError::Parse(e)
    }
}

pub fn player_move(board: &Vec<u8>) -> Result<BoardMove, MoveError> {
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
pub fn bot_move(board: &Vec<u8>) -> BoardMove {
    //find possible moves
    let mut open_moves: Vec<BoardMove> = vec![];
    for (idx, cell) in board.iter().enumerate() {
        match *cell {
            0 => open_moves.push(BoardMove { idx, symbol: 'o' }),
            _ => {}
        }
    }
    //select random
    let mut rng = rand::rng();
    match open_moves.choose(&mut rng) {
        Some(mv) => mv.clone(),
        None => panic!("no spots left"),
    }
}

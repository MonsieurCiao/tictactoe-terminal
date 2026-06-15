use crate::BoardMove;
use rand::prelude::IndexedRandom;
use std::panic;

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

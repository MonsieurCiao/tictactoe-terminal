use crate::{Board, BoardMove, check_win, update_board};
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

pub fn find_best_move(board_struct: Board, symbol: char) {
    let best_move: Option<BoardMove> = None;
    // collect all open spots
    let mut open_moves: Vec<BoardMove> = vec![];
    for (idx, cell) in board_struct.board.iter().enumerate() {
        match *cell {
            0 => open_moves.push(BoardMove { idx, symbol }),
            _ => {}
        }
    }

    // eval gamestate for all of the collected moves
    for mv in open_moves {
        //apply move
        let mut new_board = board_struct.clone();
        update_board(&mut new_board.board, mv);

        // if move found that ends game, take it.
        let score = match check_win(&mut new_board) {
            Some(1) => Some(-10),
            Some(2) => Some(10),
            Some(0) => Some(0),
            _ => None,
        };
    }
    // else search further on each branch until depth is reached
    // if no move decided, take random.

    // if no eval possible, make every choice possible and analyze those gamestates
}

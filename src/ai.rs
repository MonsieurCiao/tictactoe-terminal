use crate::{Board, BoardMove, Player, board, check_win, update_board};
use core::sync;
use rand::prelude::IndexedRandom;
use std::panic;

pub fn bot_move(board_struct: &mut Board) -> BoardMove {
    //find possible moves
    // let mut open_moves: Vec<BoardMove> = vec![];
    // for (idx, cell) in board.iter().enumerate() {
    //     match *cell {
    //         0 => open_moves.push(BoardMove { idx, symbol: 'o' }),
    //         _ => {}
    //     }
    // }
    // //select random
    // let mut rng = rand::rng();
    // match open_moves.choose(&mut rng) {
    //     Some(mv) => mv.clone(),
    //     None => panic!("no spots left"),
    // }
    //
    find_best_move(board_struct)
}

pub fn find_best_move(board_struct: &mut Board) -> BoardMove {
    // collect all open spots
    let ai_player_id = 2;
    let symbol = if ai_player_id == 1 { 'x' } else { 'o' };
    let mut open_moves: Vec<BoardMove> = vec![];
    for (idx, cell) in board_struct.board.iter().enumerate() {
        match *cell {
            0 => open_moves.push(BoardMove { idx, symbol }),
            _ => {}
        }
    }

    let mut best = i32::MIN;
    let mut best_move: Option<BoardMove> = None;
    for mv in open_moves.iter() {
        let mut board_cpy = board_struct.clone();
        update_board(
            &mut board_cpy,
            BoardMove {
                idx: mv.idx,
                symbol,
            },
        );

        //after making this move, it's the player's move
        let score = minimax(&mut board_cpy, false, 0, ai_player_id, 1);

        if score > best {
            best = score;
            best_move = Some(*mv);
        }
    }
    best_move.expect("should have found at least one move")
}

fn minimax(
    board_struct: &mut Board,
    is_maximising: bool,
    depth: i32,
    ai_player_id: u8,
    human_player_id: u8,
) -> i32 {
    match check_win(board_struct) {
        Some(winner) => match winner {
            1 if winner == ai_player_id => return 10 - depth,
            1 if winner == human_player_id => return depth - 10,
            2 if winner == ai_player_id => return 10 - depth,
            2 if winner == human_player_id => return depth - 10,
            0 => return 0,
            _ => panic!("unchecked win value"),
        },
        None => {
            let mut scores = Vec::new();
            for (idx, cell) in board_struct.board.iter().enumerate() {
                if *cell == 0 {
                    let mut board_cpy = board_struct.clone();
                    let symbol = if is_maximising {
                        if ai_player_id == 1 { 'x' } else { 'o' }
                    } else {
                        if human_player_id == 1 { 'x' } else { 'o' }
                    };
                    update_board(&mut board_cpy, BoardMove { idx, symbol });
                    let score = minimax(
                        &mut board_cpy,
                        !is_maximising,
                        depth + 1,
                        ai_player_id,
                        human_player_id,
                    );
                    scores.push(score);
                }
            }

            if scores.is_empty() {
                return 0;
            }
            if is_maximising {
                return *scores.iter().max().unwrap();
            } else {
                return *scores.iter().min().unwrap();
            }
        }
    }
}

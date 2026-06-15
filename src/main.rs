use std::{io, vec};
use tictactoe::{BoardMove, MoveError, bot_move, player_move};

fn main() {
    let mut board: Vec<u8> = vec![0; 3 * 3];

    let mut is_player_move = true;
    let mut determined_winner = false;
    while !determined_winner {
        if is_player_move {
            match player_move(&board) {
                Ok(mv) => {
                    println!("got move {:?}", mv);
                    update_board(&mut board, mv, &mut determined_winner);
                    is_player_move = false;
                }
                Err(MoveError::AlreadyOccupied) => println!("This field is already occupied!"),
                Err(MoveError::Io(e)) => println!("I/O error: {e}"),
                Err(MoveError::InvalidRow) => println!("row invalid!"),
                Err(MoveError::InvalidCol) => println!("col invalid!"),
                Err(MoveError::Parse(e)) => println!("parsing number error: {e}"),
            }
        } else {
            let mv = bot_move(&board);
            update_board(&mut board, mv, &mut determined_winner);
            is_player_move = true;
        }
    }
}

fn update_board(board: &mut Vec<u8>, board_move: BoardMove, determined_winner: &mut bool) {
    //convert symbol to number (1: player, 2: bot)
    let contestant = if board_move.symbol == 'x' { 1 } else { 2 };
    //update board
    board[board_move.idx] = contestant;
    println!("{:?}", board);
    match check_win(board) {
        Some(contestant) => {
            announce_win(contestant);
            *determined_winner = true;
        }
        _ => (),
    }
}
fn check_win(board: &Vec<u8>) -> Option<u8> {
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
    None
}
fn announce_win(contestant: u8) {
    match contestant {
        1 => {
            println!("player won!");
        }
        2 => {
            println!("bot won!");
        }
        _ => (),
    }
}

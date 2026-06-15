use std::io;
pub mod ai;
pub mod board;
pub mod error;
pub mod game;
pub mod player;

pub use ai::bot_move;
pub use board::{Board, BoardMove, check_win, update_board};
pub use error::MoveError;
pub use game::Game;
pub use player::player_move;

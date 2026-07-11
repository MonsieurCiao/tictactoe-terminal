use std::io;
pub mod ai;
pub mod app;
pub mod board;
pub mod error;
pub mod game;
pub mod input;
pub mod player;
pub mod rendering;

pub use ai::bot_move;
pub use app::App;
pub use board::{Board, BoardMove, check_win, update_board};
pub use error::MoveError;
pub use game::{Game, Player};
pub use player::player_move;

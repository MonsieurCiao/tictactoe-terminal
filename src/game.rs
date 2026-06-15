use crate::Board;
use crate::bot_move;
use crate::check_win;
use crate::player_move;
use crate::update_board;

enum Player {
    Human,
    Bot,
}
impl Player {
    fn next(&self) -> Self {
        match self {
            Self::Bot => Self::Human,
            Self::Human => Self::Bot,
        }
    }
}
pub struct Game {
    pub board: Board,
    current_player: Player,
    pub finished: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Player::Human,
            finished: false,
        }
    }

    pub fn run(&mut self) {
        while !self.finished {
            let mv = match self.current_player {
                Player::Human => match player_move(&self.board.board) {
                    Ok(mv) => mv,
                    Err(err) => {
                        println!("{:?}", err);
                        continue;
                    }
                },
                Player::Bot => bot_move(&self.board.board),
            };

            //apply move
            update_board(&mut self.board.board, mv);
            match check_win(&self.board.board) {
                Some(contestant) => {
                    Self::announce_win(contestant);
                    self.finished = true;
                }
                _ => (),
            }

            //check win
            self.current_player = self.current_player.next();
        }
    }
    pub fn announce_win(contestant: u8) {
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
}

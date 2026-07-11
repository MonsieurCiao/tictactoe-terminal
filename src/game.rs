use crate::Board;
use crate::BoardMove;
use crate::bot_move;
use crate::check_win;
use crate::player_move;
use crate::update_board;

#[derive(Debug, Eq, PartialEq)]
pub enum Player {
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
#[derive(Debug)]
pub struct Game {
    pub(crate) board: Board,
    pub(crate) current_player: Player,
    pub(crate) finished: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Player::Human,
            finished: false,
        }
    }

    pub fn apply_move(&mut self, mv: BoardMove) {
        //apply move
        update_board(&mut self.board.board, mv);

        //check win
        match check_win(&mut self.board) {
            Some(contestant) => {
                Self::announce_win(contestant);
                self.finished = true;
            }
            _ => (),
        }

        self.current_player = self.current_player.next();

        //switch to bot
        if self.current_player == Player::Bot && !self.finished {
            self.get_bot_move();
        }
    }
    fn get_bot_move(&mut self) {
        let mv = bot_move(&self.board.board);
        self.apply_move(mv);
    }
    pub fn announce_win(contestant: u8) {
        // match contestant {
        //     1 => {
        //         println!("player won!");
        //     }
        //     2 => {
        //         println!("bot won!");
        //     }
        //     _ => (),
        // }
    }
}

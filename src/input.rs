use crate::App;
use crate::BoardMove;
use crate::MoveError;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

enum GotoDir {
    Up,
    Down,
    Left,
    Right,
}
impl App {
    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Enter => self.confirm_move(),
            KeyCode::Char('j') => self.goto(GotoDir::Down),
            KeyCode::Char('k') => self.goto(GotoDir::Up),
            KeyCode::Char('h') => self.goto(GotoDir::Left),
            KeyCode::Char('l') => self.goto(GotoDir::Right),
            _ => {}
        }
    }
    fn confirm_move(&mut self)-> Result<BoardMove, MoveError> {
        if self.game.board.board[(self.position) as usize] != 0 {
            return;
        }
        // player mark at position
        return BoardMove {
            self.position
        }
        //switch to bot
    }
    fn goto(&mut self, dir: GotoDir) {
        match dir {
            GotoDir::Down => {
                if self.position <= 5 {
                    self.position += 3;
                }
            }
            GotoDir::Up => {
                if self.position > 2 {
                    self.position -= 3;
                }
            }
            GotoDir::Right => {
                if self.position < 8 {
                    self.position += 1;
                }
            }
            GotoDir::Left => {
                if self.position > 0 {
                    self.position -= 1;
                }
            }
        }
    }
}

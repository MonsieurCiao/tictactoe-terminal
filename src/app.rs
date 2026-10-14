use color_eyre::eyre::{Context, Result};
use ratatui::{DefaultTerminal, Frame};

use crate::{Board, Game};
#[derive(Debug)]
pub struct App {
    pub(crate) position: u8,
    pub(crate) game: Game,
    pub(crate) exit: bool,
}

impl App {
    pub fn new(position: u8, game: Game) -> Self {
        Self {
            position: position,
            game: game,
            exit: false,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            // if no game is runnning, start game
            if self.game.finished {
                self.game = Game::new();
            }
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    pub(crate) fn exit(&mut self) {
        self.exit = true;
    }
}

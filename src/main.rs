use color_eyre::eyre::{Context, Result};
use std::io;
use tictactoe::Game;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);

    // let mut game = Game::new();
    // game.run();

    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    //here goes the state
    position: u8,
    counter: u8,
    exit: bool,
}

enum GotoDir {
    Up,
    Down,
    Left,
    Right,
}
impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn handle_events(&mut self) -> io::Result<()> {
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
            KeyCode::Char('j') => self.goto(GotoDir::Down),
            KeyCode::Char('k') => self.goto(GotoDir::Up),
            KeyCode::Char('h') => self.goto(GotoDir::Left),
            KeyCode::Char('l') => self.goto(GotoDir::Right),
            _ => {}
        }
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
                if self.position > 0 {
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
    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Tic Tac Toe".bold());
        let instructions = Line::from(vec![
            " Left ".into(),
            "<h>".blue().bold(),
            " Down ".into(),
            "<j>".blue().bold(),
            " Up ".into(),
            "<k>".blue().bold(),
            " Right ".into(),
            "<l>".blue().bold(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
        let center = Rect::new(area.width / 2 - 30 / 2, area.height / 2 - 30 / 2, 30, 30);
        let rows = Layout::horizontal([
            Constraint::Ratio(3, 3),
            Constraint::Ratio(3, 3),
            Constraint::Ratio(3, 3),
        ])
        .split(center);

        for row in rows.iter() {
            let cols = Layout::vertical([
                Constraint::Ratio(3, 3),
                Constraint::Ratio(3, 3),
                Constraint::Ratio(3, 3),
            ])
            .split(*row);

            for cell in cols.iter() {
                //cell is a &Rect, so we render into *cell
                let block = Block::bordered();
                block.render(*cell, buf);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
    }
}

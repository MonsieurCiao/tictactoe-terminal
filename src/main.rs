use color_eyre::eyre::{Context, Result};
use std::io;
use tictactoe::{Board, Game};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut game = Game::new();
    let app_result = App::new(0, game).run(&mut terminal);

    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App {
    //here goes the state
    position: u8,
    game: Game,
    exit: bool,
}

enum GotoDir {
    Up,
    Down,
    Left,
    Right,
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

        Grid {
            rows: 3,
            cols: 3,
            board: &self.game.board,
            position: self.position.clone(),
        }
        .render(area, buf);
    }
}

struct Grid<'a> {
    rows: usize,
    cols: usize,
    board: &'a Board,
    position: u8,
}

impl<'a> Widget for Grid<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..self.cols).map(|_| Constraint::Length(8));
        let row_constraints = (0..self.rows).map(|_| Constraint::Length(4));

        let horizontal = Layout::horizontal(col_constraints);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            if i as u8 == self.position {
                Paragraph::new(format!("{:?}", self.board.board[i]))
                    .block(Block::bordered().border_style(Style::new().blue()))
                    .render(cell, buf);
            } else {
                Paragraph::new(format!("{:?}", self.board.board[i]))
                    .block(Block::bordered())
                    .render(cell, buf);
            }
        }
    }
}

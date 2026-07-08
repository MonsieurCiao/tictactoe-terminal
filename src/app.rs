use color_eyre::eyre::{Context, Result};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{Board, Game};
#[derive(Debug)]
pub struct App {
    //here goes the state
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
        Block::bordered()
            .title(title)
            .title_bottom(instructions)
            .border_set(border::THICK)
            .render(area, buf);

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

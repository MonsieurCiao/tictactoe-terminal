use color_eyre::eyre::{Context, Result};
use std::io;
use tictactoe::{App, Board, Game};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut game = Game::new();
    let app_result = App::new(0, game).run(&mut terminal);

    ratatui::restore();
    app_result
}

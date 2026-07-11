use color_eyre::eyre::Result;
use tictactoe::{App, Game};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let game = Game::new();
    let app_result = App::new(0, game).run(&mut terminal);

    ratatui::restore();
    app_result
}

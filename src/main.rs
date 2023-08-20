use std::io;

use crossterm::terminal::EnterAlternateScreen;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;

    crossterm::execute!(stdout, EnterAlternateScreen)?;
    Ok(())
}



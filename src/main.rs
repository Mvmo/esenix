use std::io;

use crossterm::terminal::EnterAlternateScreen;
use ratatui::{Terminal, prelude::CrosstermBackend, widgets::{Paragraph, Clear}};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let text_widget = Paragraph::new("Hello, World!");
    terminal.draw(|frame| {
        frame.render_widget(Clear, frame.size());
        frame.render_widget(text_widget, frame.size());
    })?;

    crossterm::execute!(stdout, EnterAlternateScreen)?;
    Ok(())
}



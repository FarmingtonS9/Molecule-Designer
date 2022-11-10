use std::{io, thread, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::{
        canvas::{Canvas, Map, MapResolution},
        Block, Borders, Paragraph, List, ListItem, ListState,
    },
    Frame, Terminal,
};

use crate::{app::App, chemistry::Atom};

pub fn tui_entry(data: Vec<Atom>) -> Result<(), io::Error> {
    let data = data;
    

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, data);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, data: Vec<Atom>) -> io::Result<()> {
    let data = data;
    let element = &data[1];

    let terminal_bg = terminal_light::background_color()
        .map(|c| c.rgb())
        .map(|c| Color::Rgb(c.r, c.g, c.b))
        .unwrap_or(Color::Gray);

    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default().direction(Direction::Vertical).margin(2).vertical_margin(2).constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()).split(size);
            let block = Block::default().title("Molecule Designer").borders(Borders::ALL);
            let paragraph = Paragraph::new(element.element.clone());

            frame.render_widget(block, size);
            frame.render_widget(paragraph, chunks[0])
        })?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

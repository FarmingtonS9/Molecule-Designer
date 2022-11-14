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
        Block, Borders, List, ListItem, ListState, Paragraph,
    },
    Frame, Terminal,
};

use crate::{chemistry::Atom};

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
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .vertical_margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);
            let block = Block::default()
                .title("Molecule Designer")
                .borders(Borders::ALL);
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

struct App {
    items: StatefulList<Vec<Atom>>,
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i==0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
    }

    fn unselect(&mut self) {
        self.state.select(None)
    }
}

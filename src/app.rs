use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct App {
    pub page: Page,
}

#[derive(PartialEq, Clone)]
pub enum Page {
    Overview,
}

impl App {
    pub fn build_layout() -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(2), Constraint::Length(3)].as_ref())
    }
    
    pub fn build_infobox(
        title: String,
        content: String,
        alignment: Alignment,
    ) -> Paragraph<'static> {
        Paragraph::new(content)
            .style(Style::default())
            .alignment(alignment)
            .block(
                Block::default()
                    .borders(Borders::all())
                    .title(title)
                    .border_type(BorderType::Plain),
            )
    }

    pub fn build_help() -> Paragraph<'static> {
        App::build_infobox(
            "Help".to_string(),
            "[/] Search | [d]ownload  | [l]ogin | [Enter]xecute | Up (k, w) | Down (j, s) | [q]uit | [Space]team"
                .to_string(),
            Alignment::Left,
        )
    }
}

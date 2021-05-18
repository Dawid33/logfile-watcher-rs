use tui::widgets::Block;
use tui::widgets::*;
use tui::text::*;
use tui::style::*;

#[derive(Clone)]
pub struct HelpBlock {}

impl HelpBlock {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view<'a>(&self) -> Paragraph<'a> {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled("Help Menu", Style::default().add_modifier(Modifier::BOLD)));
        let help_block_text = vec![
            Spans::from("This is my help text"),
        ];
        Paragraph::new(help_block_text).block(block)
    }
}
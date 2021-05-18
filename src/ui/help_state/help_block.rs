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

    pub fn view<'a>(&self, config : &super::Config) -> Paragraph<'a> {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled("Help Menu", Style::default().add_modifier(Modifier::BOLD)));
        let help_block_text = vec![
            Spans::from("Press ? to leave the help menu."),
            Spans::from(format!("Press {} to leave the program.", config.key_map.exit_key)),
        ];
        Paragraph::new(help_block_text).block(block)
    }
}
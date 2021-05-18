use tui::widgets::Block;
use tui::widgets::*;

#[derive(Clone)]
pub struct HelpBlock {
    pub text: String,
}

impl HelpBlock {
    pub fn new() -> Self {
        Self {
            text: String::from(""),
        }
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn view<'a>(&self) -> Paragraph<'a> {
        let block = Block::default().borders(Borders::ALL);
        Paragraph::new(self.text.clone()).block(block)
    }
}
use tui::widgets::Block;
use tui::widgets::*;

#[derive(Clone)]
pub struct Terminal {
    pub text: String,
    is_selected: bool,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            text: String::from(""),
            is_selected: false,
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

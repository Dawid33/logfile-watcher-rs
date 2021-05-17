use super::messages::TerminalMessage;
use tui::widgets::Block;
use tui::widgets::*;

pub struct Terminal {
    pub text: String,
}

impl Terminal {
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

    pub fn update(&mut self, message: TerminalMessage) {
        match message {
            TerminalMessage::TextUpdate(text) => self.text = text.clone(),
        }
    }
}

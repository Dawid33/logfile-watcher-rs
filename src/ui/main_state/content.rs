use tui::widgets::Block;
use tui::widgets::*;

#[derive(Clone)]
pub struct Content {
    pub text: String,
    is_selected: bool,
}

impl Content {
    pub fn new() -> Self {
        Self {
            text: String::from("Hello World!"),
            is_selected: false,
        }
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn view<'a>(&self) -> Paragraph<'a> {
        let block = Block::default()
            .borders(Borders::ALL);
        Paragraph::new(self.text.clone()).block(block)
    }
}

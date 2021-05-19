use tui::widgets::Block;
use tui::widgets::*;

#[derive(Clone)]
pub struct Content {
    pub text: Vec<String>,
    pub is_selected: bool,
}

impl Content {
    pub fn new() -> Self {
        Self {
            text: vec![String::from("Hello World!")],
            is_selected: false,
        }
    }

    pub fn text(mut self, text: Vec<String>) -> Self {
        self.text = text;
        self
    }

    pub fn view<'a>(&self) -> Paragraph<'a> {
        let block = Block::default()
            .borders(Borders::ALL);
        let drawable : Vec<tui::text::Spans> = self.text.iter().map(|i| {
            tui::text::Spans::from(tui::text::Span::from(i.clone()))
        }).collect();
        Paragraph::new(drawable).block(block)
    }
}

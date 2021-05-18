use super::Backend;
use super::events;

mod terminal;
use terminal::Terminal;

#[derive(Clone)]
pub struct UIMainState {
    terminal: Terminal,
}

impl super::UIState for UIMainState
{
    fn draw(&self, frame: &mut tui::Frame<Backend>) {
        let size = frame.size();
        frame.render_widget(self.terminal.view(), size);
    }

    fn update(
        &mut self,
        _terminal: &mut tui::Terminal<Backend>,
        event: &super::events::Event,
    ) -> Result<super::UpdateResult, Box<dyn std::error::Error>> {
        match event {
            events::Event::KeyPressed(key) => {
                match key {
                    events::Key::Char('?') => {
                        return Ok(super::UpdateResult::ReplaceUIWith(Box::new(super::UIHelpState::default())));
                    },
                    _ => (),
                }
            },
            _ => (),
        }
        Ok(super::UpdateResult::DoNothing)
    }
}

impl Default for UIMainState {
    fn default() -> Self {
        Self {
            terminal: Terminal::new().text(String::from("hello world")),
        }
    }
}

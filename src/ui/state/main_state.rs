use super::Backend;

#[derive(Clone)]
pub struct UIMainState {
    terminal: super::components::Terminal,
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
        _event: &super::events::Event,
    ) -> Result<super::UpdateResult, Box<dyn std::error::Error>> {
        Ok(super::UpdateResult::DoNothing)
    }
}

impl Default for UIMainState {
    fn default() -> Self {
        Self {
            terminal: super::Terminal::new().text(String::from("hello world")),
        }
    }
}

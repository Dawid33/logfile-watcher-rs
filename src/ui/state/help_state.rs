use super::Backend;

#[derive(Clone)]
pub struct UIHelpState {
    help_block : super::components::HelpBlock;
}

impl super::UIState for UIHelpState
{
    fn draw(&self, frame: &mut tui::Frame<Backend>) {
        let size = frame.size();
        frame.render_widget(self.help_block.view(), size);

    }

    fn update(
        &mut self,
        _terminal: &mut tui::Terminal<Backend>,
        _event: &super::events::Event,
    ) -> Result<super::UpdateResult, Box<dyn std::error::Error>> {
        Ok(super::UpdateResult::DoNothing)
    }
}

impl Default for UIHelpState {
    fn default() -> Self {
        Self {
            terminal: super::Terminal::new().text(String::from("hello world")),
        }
    }
}

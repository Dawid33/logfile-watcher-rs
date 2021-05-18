use super::Backend;
use super::events;
use super::serde::Config;

mod help_block;

#[derive(Clone)]
pub struct UIHelpState {
    pub help_block : help_block::HelpBlock,
}

impl super::UIState for UIHelpState
{
    fn draw(&mut self, frame: &mut tui::Frame<Backend>, config : &Config) {
        let size = frame.size();
        frame.render_widget(self.help_block.view(config), size);
    }

    fn update(
        &mut self,
        _terminal: &mut tui::Terminal<Backend>,
        event: &super::events::Event,
        _config: &Config,
    ) -> Result<super::UpdateResult, Box<dyn std::error::Error>> {
        match event {
            events::Event::KeyPressed(key) => {
                match key {
                    events::Key::Char('?') => {
                        return Ok(super::UpdateResult::GoToPreviousUI);
                    },
                    _ => (),
                }
            },
            _ => (),
        }
        Ok(super::UpdateResult::DoNothing)
    }
}

impl Default for UIHelpState {
    fn default() -> Self {
        Self {
            help_block: help_block::HelpBlock::new(),
        }
    }
}

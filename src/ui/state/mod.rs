pub mod main_state;
pub use main_state::UIMainState;
pub mod terminal;
pub use terminal::Terminal;
pub mod help_state;
pub use help_state::UIHelpState;
use super::messages;
use super::events;

pub trait UIState<B>
    where
        B: tui::backend::Backend,
{
    fn update(
        &mut self,
        terminal_handle: &mut tui::Terminal<B>,
        event_manager: &super::events::Event,
    ) -> Result<UpdateResult<B>, Box<dyn std::error::Error>>;

    fn draw(&self, frame: &mut tui::Frame<B>);
}

pub enum UpdateResult<B>
where 
    B: tui::backend::Backend
{
    ReplaceUIWith(Box<dyn UIState<B>>),
    DoNothing,
}

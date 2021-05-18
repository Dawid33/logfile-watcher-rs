pub mod main_state;
pub use main_state::UIMainState;
pub mod help_state;
pub use help_state::UIHelpState;

pub mod components;

use super::events;
use super::Backend;

pub trait UIState : CloneUIState
{
    fn update(
        &mut self,
        terminal_handle: &mut tui::Terminal<Backend>,
        event_manager: Box<dyn super::events::Event>,
    ) -> Result<UpdateResult, Box<dyn std::error::Error>>;

    fn draw(&self, frame: &mut tui::Frame<Backend>);
}

pub trait CloneUIState {
    fn clone_foo<'a>(&self) -> Box<dyn UIState>;
}

impl<T> CloneUIState for T
where
    T: UIState + Clone + 'static,
{
    fn clone_foo(&self) -> Box<dyn UIState> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn UIState> {
    fn clone(&self) -> Self {
        self.clone_foo()
    }
}


pub enum UpdateResult
{
    ReplaceUIWith(Box<dyn UIState>),
    GoToPreviousUI,
    DoNothing,
}
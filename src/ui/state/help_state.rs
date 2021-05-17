pub struct UIHelpState
{
    terminal : super::Terminal,
}

impl<B> super::UIState<B> for UIHelpState 
    where
        B: tui::backend::Backend,
{
    fn draw(&self, frame : &mut tui::Frame<B>){
        let size = frame.size();
        frame.render_widget(self.terminal.view(), size);
    }

    fn update(&mut self, _terminal : &mut tui::Terminal<B>, _event : &super::events::Event) -> Result<super::UpdateResult<B>, Box<dyn std::error::Error>> {
        Ok(super::UpdateResult::DoNothing)
    }
}

impl Default for UIHelpState {
    fn default() -> Self {
        Self {
            terminal : super::Terminal::new().text(String::from("hello world")),
        }
    }
}
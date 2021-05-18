use super::Backend;
use super::events;
use super::serde::Config;

mod content;
use content::Content;
mod sidebar;
use sidebar::Sidebar;
mod list;
use list::StatefulList;

use tui::layout::*;

#[derive(Clone)]
pub struct UIMainState {
    content: Content,
    sidebar: Sidebar,
    last_selected_item: Option<usize>,
    size : (u16, u16),
}

impl super::UIState for UIMainState
{
    fn draw(&mut self, frame: &mut tui::Frame<Backend>, _config: &Config) {
        let size = frame.size();
        let chunks = tui::layout::Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(self.size.0),
                Constraint::Percentage(self.size.1),
            ]).split(size);

        let list = self.sidebar.view();
        frame.render_stateful_widget(list, chunks[0], &mut self.sidebar.items.state);
        frame.render_widget(self.content.view(), chunks[1]);
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
                        return Ok(super::UpdateResult::ReplaceUIWith(Box::new(super::UIHelpState::default())));
                    },
                    events::Key::Char('H') => {
                        if self.size.0 > 0 {
                            self.size.0 -= 2;
                            self.size.1 += 2;
                        }
                    }
                    events::Key::Char('L') => {
                        if self.size.1 > 0 {
                            self.size.1 -= 2;
                            self.size.0 += 2;
                        }
                    }
                    events::Key::Char('h') => {
                        if let Some(i) = self.last_selected_item {
                            self.sidebar.items.select(i);
                        } else {
                            self.sidebar.items.select(1);
                        }
                    }
                    events::Key::Char('l') => {
                        self.last_selected_item = self.sidebar.items.state.selected();
                        self.sidebar.items.unselect();
                    }
                    events::Key::Char('j') => {
                        self.sidebar.items.next();
                    }
                    events::Key::Char('k') => {
                        self.sidebar.items.previous();
                    }
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
            content: Content::new(),
            sidebar: Sidebar::new(),
            last_selected_item : None,
            size : (20,80)
        }
    }
}

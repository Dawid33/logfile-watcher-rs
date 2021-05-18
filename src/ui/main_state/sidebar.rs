use tui::widgets::Block;
use tui::widgets::*;
use tui::style::*;
use tui::text::*;

#[derive(Clone)]
pub struct Sidebar {
    pub items : super::StatefulList<(url::Url,String)>,
    pub is_selected: bool,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            items : super::StatefulList::with_items(vec![
                (url::Url::from_file_path(std::env::current_dir().unwrap().join(std::path::Path::new("assets/testing1.txt"))).unwrap(),String::from("First")),
                (url::Url::from_file_path(std::env::current_dir().unwrap().join(std::path::Path::new("assets/testing2.txt"))).unwrap(),String::from("Second")),
                (url::Url::from_file_path(std::env::current_dir().unwrap().join(std::path::Path::new("assets/testing3.txt"))).unwrap(),String::from("Third")),
            ]),
            is_selected : false,
        }
    }

    pub fn view<'a>(&self) -> tui::widgets::List<'a> {
        let block = Block::default().borders(Borders::BOTTOM | Borders::LEFT | Borders::TOP);
        let items : Vec<ListItem> = self.items.items.iter().map(|i| {
            let item =  Spans::from(i.1.clone());
            ListItem::new(item)
        })
        .collect();

        List::new(items)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::LightBlue))
    }
}

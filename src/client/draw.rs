use common::json_structs::ClientConfig;
use super::ui;

use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span,Spans},
    widgets::{Block, BorderType, Borders,Paragraph},
    backend::Backend,
};

pub fn draw_client<B>(
    terminal: &mut tui::Terminal<B>,
    client_config: &mut ClientConfig,
    state : &ui::UIState,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: Backend,
{
    terminal.draw(|frame| {        
        let size = frame.size();
        let outside_border = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(client_config.ui_config.background_color));
        frame.render_widget(outside_border, size);

        let sidebar_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80)
                ]
                .as_ref(),
            )
            .split(frame.size());
        let sidebar = Block::default()
            .border_style(Style::default().fg(Color::Cyan))
            .borders(Borders::RIGHT)
            .border_type(BorderType::Thick);
        frame.render_widget(sidebar, sidebar_layout[0]);

        let text = vec![
            Spans::from("This is one sentence."),
            Spans::from("This is another sentence."),
        ];
        let paragraph = Paragraph::new(text.clone());
        let main_bar = Block::default()
            .style(Style::default().fg(Color::Blue))
            .title(Span::from(
                match &state.current_file_path {
                    Some(x) => x.file_name().unwrap().to_str().unwrap(),
                    None => "Viewer"
                }));

        let main_panel_layout = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(2)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ]
                .as_ref(),
            )
            .split(frame.size());
        let paragraph_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ]
                .as_ref(),
            )
            .split(frame.size());
        
        frame.render_widget(paragraph, paragraph_layout[1]);
        frame.render_widget(main_bar, main_panel_layout[1]);
        
    })?;
    Ok(())
}
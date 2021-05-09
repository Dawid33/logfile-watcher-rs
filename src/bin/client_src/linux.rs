use super::*;

use std::{
    io::Write,
};

use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
};

pub fn draw_client<W>(
    terminal: &mut tui::Terminal<tui::backend::TermionBackend<W>>, 
    draw_config : &ui::DrawConfig,
) -> Result<(), Box<dyn std::error::Error>> 
where
    W: Write,
{
    terminal.draw(|frame| {
        let size = frame.size();
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        frame.render_widget(block, size);
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100),Constraint::Percentage(100)].as_ref())
            .split(frame.size());
        
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(draw_config.sidebar_percentage_size), Constraint::Percentage(draw_config.main_panel_percentage_size)].as_ref())
            .split(chunks[0]);
        let block = Block::default()
            .title(vec![
                Span::from("With background"),
            ]);
            frame.render_widget(block, top_chunks[0]);
    
        let block = Block::default().title(Span::styled(
            "Styled title",
            Style::default()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ));
        frame.render_widget(block, top_chunks[1]);
    })?;
    Ok(())
}
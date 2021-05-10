use super::ui;
use common::json_structs::ClientConfig;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn draw_client<B>(
    terminal: &mut tui::Terminal<B>,
    client_config: &ClientConfig,
    ui_state: &ui::UIState,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: Backend,
{
    match ui_state.current_mode {
        ui::UIMode::Main => draw_main(terminal, client_config, ui_state)?,
        ui::UIMode::Help => draw_help(terminal, client_config, ui_state)?,
    }
    Ok(())
}

pub fn draw_main<B>(
    terminal: &mut tui::Terminal<B>,
    client_config: &ClientConfig,
    ui_state: &ui::UIState,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: Backend,
{
    terminal.draw(|frame| {
        let size = frame.size();
        let outside_border = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(rgb_tuple_to_color(
                &client_config.ui_config.background_color,
            )));
        frame.render_widget(outside_border, size);

        let sidebar_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(ui_state.percent_size_of_panes.0),
                    Constraint::Percentage(ui_state.percent_size_of_panes.1),
                ]
                .as_ref(),
            )
            .split(frame.size());
        let sidebar = Block::default()
            .border_style(Style::default())
            .borders(Borders::RIGHT)
            .border_type(BorderType::Thick);
        frame.render_widget(sidebar, sidebar_layout[0]);

        let text = vec![
            Spans::from("This is one sentence."),
            Spans::from("This is another sentence."),
        ];
        let paragraph = Paragraph::new(text.clone());
        let main_bar = Block::default().title(Span::from(match &ui_state.current_file_path {
            Some(x) => String::from(x.file_name().unwrap().to_str().unwrap()),
            None => ui_state.default_main_panel_title.clone(),
        }));

        let main_panel_layout = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(2)
            .constraints(
                [
                    Constraint::Percentage(ui_state.percent_size_of_panes.0),
                    Constraint::Percentage(ui_state.percent_size_of_panes.1),
                ]
                .as_ref(),
            )
            .split(frame.size());
        let paragraph_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(ui_state.percent_size_of_panes.0),
                    Constraint::Percentage(ui_state.percent_size_of_panes.1),
                ]
                .as_ref(),
            )
            .split(frame.size());

        frame.render_widget(paragraph, paragraph_layout[1]);
        frame.render_widget(main_bar, main_panel_layout[1]);
    })?;
    Ok(())
}

pub fn draw_help<B>(
    terminal: &mut tui::Terminal<B>,
    client_config: &ClientConfig,
    _ui_state: &ui::UIState,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: Backend,
{
    terminal.draw(|frame| {
        let size = frame.size();
        let outside_border = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(rgb_tuple_to_color(
                &client_config.ui_config.background_color,
            )));
        
        let text = vec![
            Spans::from("This is the help menu."),
            Spans::from(format!("{} : quit the help menu", client_config.key_map.help)),
            Spans::from(format!("{} : quit the application", client_config.key_map.quit)),
            Spans::from(format!("{} and {} : resize the panes left and right", client_config.key_map.left,client_config.key_map.right)),
        ];
        let paragraph = Paragraph::new(text.clone());
        let paragraph_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref(),)
            .split(frame.size());

        frame.render_widget(outside_border, size);
        frame.render_widget(paragraph, paragraph_layout[0]);
    })?;
    Ok(())
}
pub fn rgb_tuple_to_color(rgb: &(u8, u8, u8)) -> Color {
    Color::Rgb(rgb.0, rgb.1, rgb.2)
}

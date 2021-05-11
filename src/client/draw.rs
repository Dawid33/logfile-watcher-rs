use super::ui;
use common::json_structs::ClientConfig;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, ListItem, List},
};

pub fn draw_client<B>(
    terminal: &mut tui::Terminal<B>,
    client_config: &ClientConfig,
    ui_state: &mut ui::UIState,
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
    ui_state: &mut ui::UIState,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: Backend,
{
    terminal.draw(|frame| {
        let size = frame.size();
        let items: Vec<ListItem> = ui_state.sidebar_list
            .items
            .iter()
            .map(|i| {
                
                let line = Spans::from(Span::styled(
                    i.clone(),
                    Style::default().add_modifier(Modifier::ITALIC),
                ));
                ListItem::new(line).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        let sidebar_list = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(ui_state.percent_size_of_panes.0),
                    Constraint::Percentage(ui_state.percent_size_of_panes.1),
                ]
                .as_ref(),
            )
            .split(frame.size());
            
        let content_panel = Block::default().title(Span::from(match &ui_state.current_file_path {
            Some(x) => String::from(x.file_name().unwrap().to_str().unwrap()),
            None => ui_state.debug.clone(),
        })).borders(Borders::ALL);

        let text = vec![
                Spans::from("This is one sentence."),
                Spans::from("This is another sentence."),
            ];
        
        let content = Paragraph::new(text.clone()).block(content_panel);

        let content_panel_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(ui_state.percent_size_of_panes.0),
                    Constraint::Percentage(ui_state.percent_size_of_panes.1),
                ]
                .as_ref(),
            )
            .split(size);
            
        //frame.render_widget(outside_border, size);
        //frame.render_widget(sidebar, sidebar_layout[0]);
        //frame.render_widget(paragraph, paragraph_layout[1]);
        frame.render_widget(content, content_panel_layout[1]);
        frame.render_stateful_widget(items, sidebar_list[0], &mut ui_state.sidebar_list.state);
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
        
        let text = vec![
            Spans::from(format!("{} : quit the help menu", client_config.key_map.help)),
            Spans::from(format!("{} : quit the application", client_config.key_map.quit)),
            Spans::from(format!("{} : reload the config file [{}]", client_config.key_map.reload_config, super::CONFIG_FILENAME)),
            Spans::from(format!("{} and {} : resize the panes left and right", client_config.key_map.left,client_config.key_map.right)),
        ];
        let help_block = Block::default()
            .title(Span::styled("Help Menu\n", Style::default().add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL);
        let paragraph = Paragraph::new(text.clone()).block(help_block);
        let help_block_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref(),)
            .split(frame.size());

        frame.render_widget(paragraph, help_block_layout[0]);
    })?;
    Ok(())
}
pub fn rgb_tuple_to_color(rgb: &(u8, u8, u8)) -> Color {
    Color::Rgb(rgb.0, rgb.1, rgb.2)
}
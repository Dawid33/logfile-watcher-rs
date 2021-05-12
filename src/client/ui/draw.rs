use {
    super::*,
    common::json_structs::ClientConfig,
    tui::{
        backend::Backend,
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        text::{Span, Spans},
        widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    },
};

/**
 * ## draw_client()
 *  Draw the client using the handle to the `terminal`. tui works by creating instances of
 *  "widgets" and passing them into a render function. The `ui_state` and `client_config`
 *  dictate how to create these widgets.
 */
pub fn draw_client<B>(
    terminal: &mut tui::Terminal<B>,
    client_config: &ClientConfig,
    ui_state: &mut UIState,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: Backend,
{
    match ui_state.current_mode {
        UIMode::Main => draw_main(terminal, client_config, ui_state)?,
        UIMode::Help => draw_help(terminal, client_config, ui_state)?,
    }
    Ok(())
}

pub fn draw_main<B>(
    terminal: &mut tui::Terminal<B>,
    _client_config: &ClientConfig,
    ui_state: &mut UIState,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: Backend,
{
    terminal.draw(|frame| {
        let size = frame.size();
        //Create a vec of ListItems (text objects) from the current ui_state.
        let items: Vec<ListItem> = ui_state
            .sidebar_list
            .items
            .iter()
            .map(|i| {
                let line = Spans::from(Span::styled(i.1.clone(), Style::default()));
                ListItem::new(line).style(Style::default())
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            );

        // Layout for the sidebar
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

        // Main content pane where data is displayed
        let content_panel = Block::default()
            .title(Span::from(ui_state.current_content_panel_title.clone()))
            .borders(Borders::RIGHT | Borders::BOTTOM | Borders::TOP);

        //Main content pane + the content to put inside it.
        let content = Paragraph::new(ui_state.content.clone())
            .block(content_panel)
            .wrap(Wrap { trim: false });

        //Main content pane layout
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
        //Render sidebar and content pane.
        frame.render_widget(content, content_panel_layout[1]);
        frame.render_stateful_widget(items, sidebar_list[0], &mut ui_state.sidebar_list.state);
    })?;
    Ok(())
}

/// Draw the help menu.
pub fn draw_help<B>(
    terminal: &mut tui::Terminal<B>,
    client_config: &ClientConfig,
    _ui_state: &UIState,
) -> Result<(), Box<dyn std::error::Error>>
where
    B: Backend,
{
    terminal.draw(|frame| {
        let text = vec![
            Spans::from(format!(
                "{:?} : quit the help menu.",
                client_config.key_map.help
            )),
            Spans::from(format!(
                "{:?} : quit the application.",
                client_config.key_map.quit
            )),
            Spans::from(format!(
                "{:?} : reload the config file [{}].",
                client_config.key_map.reload_config,
                super::CONFIG_FILENAME
            )),
            Spans::from(format!(
                "{:?} and {:?}: Move up and down the file menu.",
                client_config.key_map.up, client_config.key_map.down
            )),
            Spans::from(format!(
                "{:?} and {:?}: Move between the file menu and main panel.",
                client_config.key_map.left, client_config.key_map.right
            )),
            Spans::from(format!(
                "{:?} and {:?} : resize the panes left and right.",
                client_config.key_map.resize_left, client_config.key_map.resize_right
            )),
        ];
        let help_block = Block::default()
            .title(Span::styled(
                "Help Menu\n",
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL);
        let paragraph = Paragraph::new(text.clone()).block(help_block);
        let help_block_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame.size());

        frame.render_widget(paragraph, help_block_layout[0]);
    })?;
    Ok(())
}

pub fn rgb_tuple_to_color(rgb: &(u8, u8, u8)) -> Color {
    Color::Rgb(rgb.0, rgb.1, rgb.2)
}

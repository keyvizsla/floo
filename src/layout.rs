use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Spacing},
    style::{Color, Modifier, Style, Stylize},
    symbols::merge::MergeStrategy,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::state::AppState;

/// Module handles all the layout and drawing of the app screen

/// Draw the main app screen onto the given frame.
pub fn draw(frame: &mut Frame, state: &mut AppState) {
    if state.projects.len() > 0 {
        draw_with_projects(frame, state);
    } else {
        draw_empty_state_screen(frame);
    }
}

pub fn draw_empty_state_screen(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(12),
            Constraint::Percentage(30),
        ])
        .split(f.area());

    let ascii_logo = r#"
███████╗██╗      ██████╗  ██████╗ 
██╔════╝██║     ██╔═══██╗██╔═══██╗
█████╗  ██║     ██║   ██║██║   ██║
██╔══╝  ██║     ██║   ██║██║   ██║
██║     ███████╗╚██████╔╝╚██████╔╝
╚═╝     ╚══════╝ ╚═════╝  ╚═════╝ "#;

    let version_hint = Span::styled(
        " v0.0.1",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
    );

    let status_line = Line::from("Your floo network is dark, no fireplaces are connected.").bold();
    let hint_line = Line::from(vec![
        Span::raw("To add a destination, press "),
        Span::styled("n", Style::default().bold().fg(Color::LightCyan)),
        Span::raw(" or "),
        Span::styled("%", Style::default().bold().fg(Color::LightCyan)),
    ])
    .fg(Color::DarkGray);

    let mut final_content = Text::from(ascii_logo);
    final_content.lines.push(Line::from(version_hint));
    final_content.lines.push(Line::from("")); // Spacer
    final_content.lines.push(status_line);
    final_content.lines.push(hint_line);

    let paragraph = Paragraph::new(final_content).alignment(Alignment::Center);

    f.render_widget(paragraph, chunks[1]);
}

/// Draw the main screen given that the state contains at least 1 project.
fn draw_with_projects(frame: &mut Frame, state: &mut AppState) {
    let list = component_project_list(state.clone());
    let [left, right] = Layout::horizontal([Constraint::Fill(1); 2])
        .spacing(Spacing::Overlap(1))
        .areas(frame.area());

    frame.render_stateful_widget(list, left, &mut state.project_list_state);

    let raw_project_description = state
        .selected_project()
        .unwrap()
        .get_description()
        .unwrap_or_else(|| "No description available.".to_string());
    frame.render_widget(
        component_project_description(&raw_project_description),
        right,
    );
}

/// Generate the list to be displayed on the left side of the screen
/// listing all of the projects.
fn component_project_list(state: AppState) -> List<'static> {
    let list_items: Vec<ListItem> = state
        .projects
        .iter()
        .map(|i| ListItem::new(i.name.clone()))
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .title(" Select an Item (j/k or Arrows) ")
                .borders(Borders::ALL)
                .merge_borders(MergeStrategy::Exact),
        )
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);

    return list;
}

/// Return the given text rendered, assuming it is a markdown project description.
/// TODO: Make long text scrollable
fn component_project_description<'a>(text: &'a str) -> Paragraph<'a> {
    let parsed_text = tui_markdown::from_str(text);

    Paragraph::new(parsed_text)
        .block(
            Block::bordered()
                .title("Project Description")
                .merge_borders(MergeStrategy::Exact),
        )
        .wrap(Wrap { trim: false })
}

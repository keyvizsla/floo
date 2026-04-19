use ratatui::{
    Frame,
    layout::{Constraint, Layout, Spacing},
    symbols::merge::MergeStrategy,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::state::AppState;

/// Module handles all the layout and drawing of the app screen

/// Draw the main app screen onto the given frame.
pub fn draw(frame: &mut Frame, state: &mut AppState) {
    let list = component_project_list(state.clone());
    let [left, right] = Layout::horizontal([Constraint::Fill(1); 2])
        .spacing(Spacing::Overlap(1))
        .areas(frame.area());

    frame.render_stateful_widget(list, left, &mut state.project_list_state);
    frame.render_widget(component_project_description(state.clone()), right);
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

fn component_project_description(state: AppState) -> Paragraph<'static> {
    let text = state
        .selected_project()
        .unwrap()
        .get_description()
        .unwrap_or_else(|| "No description available.".to_string());

    let text_elem = Paragraph::new(text).block(
        Block::bordered()
            .title("Project Description")
            .merge_borders(MergeStrategy::Exact),
    );

    return text_elem;
}

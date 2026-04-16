use ratatui::{
    Frame,
    layout::{Constraint, Layout, Spacing},
    symbols::merge::MergeStrategy,
    widgets::{Block, Borders, List, ListItem},
};

use crate::state::AppState;

/// Module handles all the layout and drawing of the app screen

pub fn draw(frame: &mut Frame, state: &mut AppState) {
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

    let [left, right] = Layout::horizontal([Constraint::Fill(1); 2])
        .spacing(Spacing::Overlap(1))
        .areas(frame.area());

    frame.render_stateful_widget(list, left, &mut state.project_list_state);
    frame.render_widget(
        Block::bordered()
            .title("Right")
            .merge_borders(MergeStrategy::Exact),
        right,
    );
}

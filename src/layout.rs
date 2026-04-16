use ratatui::{
    Frame,
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
                .borders(Borders::ALL),
        )
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, frame.area(), &mut state.project_list_state);
}

use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::Clear;
use ratatui_interact::components::{DialogConfig, DialogFocusTarget, DialogState, Input, InputState};
use ratatui_interact::events::{get_char, is_backspace, is_delete, is_end, is_home};
use ratatui_interact::prelude::PopupDialog;
use crate::action::Action;
use crate::components::component::{Component, ComponentCreationError};

pub struct NewFireplaceComponent {
    dialog_state: DialogState<PopupContent>
}

impl NewFireplaceComponent {
    pub fn new() -> Self {
        Self {
            dialog_state: DialogState::new(PopupContent::default()),
        }
    }
    fn handle_dialog_content_key(&mut self, key_code: KeyCode, key: &crossterm::event::KeyEvent) {
        // Copy focus target to avoid borrow issues
        let focus_target = self.dialog_state.current_focus().cloned();

        if let Some(DialogFocusTarget::Child(idx)) = focus_target {
            let content = &mut self.dialog_state.children;
            match idx {
                0 => {
                    // Username input
                    if let Some(c) = get_char(key) {
                        content.name.insert_char(c);
                    } else if is_backspace(key) {
                        content.name.delete_char_backward();
                    } else if is_delete(key) {
                        content.name.delete_char_forward();
                    } else if key_code == KeyCode::Left {
                        content.name.move_left();
                    } else if key_code == KeyCode::Right {
                        content.name.move_right();
                    } else if is_home(key) {
                        content.name.move_home();
                    } else if is_end(key) {
                        content.name.move_end();
                    }
                }
                1 => {
                    // Email input
                    if let Some(c) = get_char(key) {
                        content.directory.insert_char(c);
                    } else if is_backspace(key) {
                        content.directory.delete_char_backward();
                    } else if is_delete(key) {
                        content.directory.delete_char_forward();
                    } else if key_code == KeyCode::Left {
                        content.directory.move_left();
                    } else if key_code == KeyCode::Right {
                        content.directory.move_right();
                    } else if is_home(key) {
                        content.directory.move_home();
                    } else if is_end(key) {
                        content.directory.move_end();
                    }
                }
                _ => {}
            }
        }
    }
    fn render_dialog(&mut self, f: &mut Frame, area: Rect) {
        // Compute focus states first
        let focus_states = [
            self.dialog_state
                .focus
                .is_focused(&DialogFocusTarget::Child(0)),
            self.dialog_state
                .focus
                .is_focused(&DialogFocusTarget::Child(1)),
        ];

        let config = DialogConfig::new("Create New Fireplace")
            .max_size(area.width, area.height)
            .ok_cancel();
        let mut dialog = PopupDialog::new(
            &config,
            &mut self.dialog_state,
            |frame, area, content| {
                Self::render_settings_content(frame, area, content, &focus_states);
            },
        );
        dialog.render(f);
    }

    fn render_settings_content(
        f: &mut Frame,
        area: Rect,
        content: &mut PopupContent,
        focus_states: &[bool; 2],
    ) {
        // Layout: inputs then checkboxes
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Username
                Constraint::Length(3), // Email
                Constraint::Min(0),    // Remaining space
            ])
            .split(area);

        content.name.focused = focus_states[0];
        let input = Input::new(&content.name).label("Fireplace Name");
        let region = input.render_stateful(f, chunks[0]);

        content.directory.focused = focus_states[1];
        let input = Input::new(&content.directory).label("Path to Fireplace");
        let region = input.render_stateful(f, chunks[1]);
    }
}

impl Component for NewFireplaceComponent {
    fn init(&mut self) -> Result<(), ComponentCreationError> {
        for i in 0..PopupContent::num_fields() {
            self.dialog_state.register_child(i);
        }
        self.dialog_state.register_button(0);
        self.dialog_state.register_button(1);
        self.dialog_state.focus.set(DialogFocusTarget::Child(0));
        self.dialog_state.show();
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Action {
        if event.is_none() {
            return Action::Noop;
        }

        match event.unwrap() {
            Event::Key(e) => self.handle_key_events(e),
            Event::Mouse(m) => self.handle_mouse_events(m),
            _ => Action::Noop,
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        self.handle_dialog_content_key(key.code, &key);

        // TODO: Needs to eventually emit project creation and such
        return Action::Noop;
    }

    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action {
        return Action::Noop;
    }

    fn update(&mut self, action: Action) -> Action {
        return Action::Noop;
    }

    fn render(&mut self, f: &mut Frame, rect: Rect) {
            //f.render_widget(Clear, rect);
            self.render_dialog(f, rect);
    }
}

#[derive(Default)]
struct PopupContent {
    name: InputState,
    directory: InputState,
}

impl PopupContent {
    fn num_fields() -> usize {
        2
    }
}
/*
 * Copyright (C) 2026 Leon Degel-Koehn
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::action::Action;
use crate::components::component::{Component, ComponentCreationError};
use crate::fireplace::Fireplace;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui_interact::components::{
    DialogConfig, DialogFocusTarget, DialogState, Input, InputState,
};
use ratatui_interact::events::{
    get_char, is_backspace, is_delete, is_end, is_enter, is_home, is_tab,
};
use ratatui_interact::prelude::PopupDialog;
use std::path::PathBuf;

pub struct NewFireplaceComponent {
    dialog_state: DialogState<PopupContent>,
}

impl NewFireplaceComponent {
    pub fn new() -> Self {
        Self {
            dialog_state: DialogState::new(PopupContent::default()),
        }
    }

    pub fn with_prefill(project: Fireplace) -> Self {
        let mut popup_content = PopupContent::default();
        popup_content.name.text = project.name.clone();
        popup_content.directory.text = project.get_directory().to_str().unwrap().to_string();
        Self {
            dialog_state: DialogState::new(popup_content),
        }
    }

    // Return the Action that is to be performed by the containing component, e.g. the start screen
    fn handle_dialog_content_key(
        &mut self,
        key_code: KeyCode,
        key: &crossterm::event::KeyEvent,
    ) -> Action {
        // Copy focus target to avoid borrow issues
        let focus_target = self.dialog_state.current_focus().cloned();

        if let Some(DialogFocusTarget::Child(idx)) = focus_target {
            let content = &mut self.dialog_state.children;
            match idx {
                0 => {
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
                    } else if is_tab(key) || key_code == KeyCode::Down {
                        self.dialog_state.focus.set(DialogFocusTarget::Child(1));
                    }
                }
                1 => {
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
                    } else if is_tab(key) || key_code == KeyCode::Down {
                        self.dialog_state.focus.set(DialogFocusTarget::Button(0));
                    } else if key_code == KeyCode::Up {
                        self.dialog_state.focus.set(DialogFocusTarget::Child(0));
                    }
                }
                _ => {}
            }
        }

        if let Some(DialogFocusTarget::Button(idx)) = focus_target {
            let content = &mut self.dialog_state.children;
            match idx {
                0 => {
                    if is_tab(key) || key_code == KeyCode::Right {
                        self.dialog_state.focus.set(DialogFocusTarget::Button(1));
                    } else if key_code == KeyCode::Up {
                        self.dialog_state.focus.set(DialogFocusTarget::Child(1));
                    } else if is_enter(key) {
                        return Action::ClosePopup;
                    }
                }
                1 => {
                    if key_code == KeyCode::Left {
                        self.dialog_state.focus.set(DialogFocusTarget::Button(0));
                    } else if is_tab(key) || key_code == KeyCode::Right {
                        self.dialog_state.focus.set(DialogFocusTarget::Child(0));
                    } else if key_code == KeyCode::Up {
                        self.dialog_state.focus.set(DialogFocusTarget::Child(1));
                    } else if is_enter(key) {
                        let new_project = Fireplace::new(
                            content.name.text.clone(),
                            PathBuf::from(content.directory.text.clone())
                                .canonicalize()
                                .unwrap_or_else(|_| PathBuf::from(content.directory.text.clone())),
                            String::new(),
                            0,
                        );
                        return Action::AddFireplace(new_project);
                    }
                }
                _ => {}
            }
        }

        Action::Noop
    }

    fn render_dialog(&mut self, f: &mut Frame, area: Rect) {
        let focus_states = [
            self.dialog_state
                .focus
                .is_focused(&DialogFocusTarget::Child(0)),
            self.dialog_state
                .focus
                .is_focused(&DialogFocusTarget::Child(1)),
        ];

        let required_height = 12;
        let max_height = if area.height < required_height {
            area.height
        } else {
            required_height
        };

        let config = DialogConfig::new("Create a new Fireplace")
            .max_size(area.width, max_height)
            .ok_cancel();
        let mut dialog =
            PopupDialog::new(&config, &mut self.dialog_state, |frame, area, content| {
                Self::render_popup_content(frame, area, content, &focus_states);
            });
        dialog.render(f);
    }

    fn render_popup_content(
        f: &mut Frame,
        area: Rect,
        content: &mut PopupContent,
        focus_states: &[bool; 2],
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Fireplace name
                Constraint::Length(3), // Directory
                Constraint::Min(0),    // Confirmation buttons
            ])
            .split(area);

        content.name.focused = focus_states[0];
        let input = Input::new(&content.name).label("Fireplace Name");
        let _region = input.render_stateful(f, chunks[0]);

        content.directory.focused = focus_states[1];
        let input = Input::new(&content.directory).label("Path to Fireplace");
        let _region = input.render_stateful(f, chunks[1]);
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

    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        self.handle_dialog_content_key(key.code, &key)
    }

    fn handle_mouse_events(&mut self, _mouse: MouseEvent) -> Action {
        Action::Noop
    }

    fn update(&mut self, _action: Action) -> Action {
        Action::Noop
    }

    fn render(&mut self, f: &mut Frame, rect: Rect) {
        self.render_dialog(f, rect);
    }
}

#[derive(Default)]
struct PopupContent {
    name: InputState,

    // TODO: It might be nicer to have a real file picker here, rather than having to input the directory manually.
    directory: InputState,
}

impl PopupContent {
    fn num_fields() -> usize {
        2
    }
}

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
use crate::project::Project;
use crate::utils::longest_line;
use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui_interact::components::{DialogConfig, DialogFocusTarget, DialogState};
use ratatui_interact::events::{is_enter, is_tab};
use ratatui_interact::prelude::PopupDialog;

pub struct DeletionPopup {
    dialog_state: DialogState<PopupContent>,
    target_project: Project,
}

impl DeletionPopup {
    pub fn new(target_project: Project) -> Self {
        Self {
            dialog_state: DialogState::new(PopupContent::default()),
            target_project,
        }
    }

    fn popup_text(&self) -> String {
        format!(
            "Are you sure you want to delete the fireplace \"{}\"?",
            self.target_project.name
        )
    }

    fn handle_dialog_content_key(
        &mut self,
        key_code: KeyCode,
        key: &crossterm::event::KeyEvent,
    ) -> Action {
        // Copy focus target to avoid borrow issues
        let focus_target = self.dialog_state.current_focus().cloned();

        if let Some(DialogFocusTarget::Button(idx)) = focus_target {
            match idx {
                0 => {
                    if is_tab(key) || key_code == KeyCode::Right {
                        self.dialog_state.focus.set(DialogFocusTarget::Button(1));
                    } else if is_enter(key) {
                        return Action::ClosePopup;
                    }
                }
                1 => {
                    if is_tab(key) || key_code == KeyCode::Left {
                        self.dialog_state.focus.set(DialogFocusTarget::Button(0));
                    } else if is_enter(key) {
                        return Action::DeleteFireplace(self.target_project.clone());
                    }
                }
                _ => {}
            }
        }

        return Action::Noop;
    }

    fn height(&self, area: Rect) -> u16 {
        let required_height = 6;
        if area.height < required_height {
            area.height
        } else {
            required_height
        }
    }

    fn width(&self, area: Rect) -> u16 {
        let required_width = 2 + longest_line(&self.popup_text()) as u16;
        if area.width < required_width {
            area.width
        } else {
            required_width
        }
    }

    fn render_dialog(&mut self, f: &mut Frame, area: Rect) {
        let text = self.popup_text();
        let config = DialogConfig::new("Delete Fireplace")
            .max_size(self.width(area), self.height(area))
            .yes_no();
        let mut dialog =
            PopupDialog::new(&config, &mut self.dialog_state, |frame, area, _content| {
                Self::render_popup_content(frame, area, &text);
            });
        dialog.render(f);
    }

    fn render_popup_content(f: &mut Frame, area: Rect, text: &str) {
        let text = Paragraph::new(text).wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(text, area);
    }
}

impl Component for DeletionPopup {
    fn init(&mut self) -> Result<(), ComponentCreationError> {
        self.dialog_state.register_button(0);
        self.dialog_state.register_button(1);
        self.dialog_state.focus.set(DialogFocusTarget::Button(0));
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
        self.handle_dialog_content_key(key.code, &key)
    }

    fn handle_mouse_events(&mut self, _mouse: MouseEvent) -> Action {
        return Action::Noop;
    }

    fn update(&mut self, _action: Action) -> Action {
        return Action::Noop;
    }

    fn render(&mut self, f: &mut Frame, rect: Rect) {
        self.render_dialog(f, rect);
    }
}

#[derive(Default)]
struct PopupContent {}

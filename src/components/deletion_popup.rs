use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Clear, Paragraph};
use ratatui_interact::components::{DialogConfig, DialogFocusTarget, DialogState, Input, InputState};
use ratatui_interact::events::{get_char, is_backspace, is_delete, is_end, is_enter, is_home, is_tab};
use ratatui_interact::prelude::{PopupDialog, SearchState};
use crate::action::Action;
use crate::components::component::{Component, ComponentCreationError};
use crate::project::Project;

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
        format!("Are you sure you want to delete the fireplace \"{}\"?", self.target_project.name)
    }

    fn handle_dialog_content_key(&mut self, key_code: KeyCode, key: &crossterm::event::KeyEvent) -> Action {
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
    fn render_dialog(&mut self, f: &mut Frame, area: Rect) {
        let config = DialogConfig::new("Delete Fireplace")
            .max_size(area.width, area.height)
            .ok_cancel();
        let text = self.popup_text();
        let mut dialog = PopupDialog::new(
            &config,
            &mut self.dialog_state,
            |frame, area, content| {
                Self::render_popup_content(frame, area, &text);
            },
        );
        dialog.render(f);
    }

    fn render_popup_content(
        f: &mut Frame,
        area: Rect,
        text: &str,
    ) {
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

    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action {
        return Action::Noop;
    }

    fn update(&mut self, action: Action) -> Action {
        return Action::Noop;
    }

    fn render(&mut self, f: &mut Frame, rect: Rect) {
        self.render_dialog(f, rect);
    }
}

#[derive(Default)]
struct PopupContent {}
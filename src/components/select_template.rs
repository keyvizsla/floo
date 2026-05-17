use std::path::PathBuf;

use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use ratatui::{Frame, layout::Rect};
use ratatui_interact::{
    components::file_explorer::{FileExplorerMode, FooterBuilder},
    prelude::{
        DialogConfig, DialogFocusTarget, DialogState, FileExplorer, FileExplorerState, PopupDialog,
    },
};

use crate::{
    action::Action,
    components::component::{Component, ComponentCreationError},
};

pub struct SelectTemplatePopup {
    dialog_state: DialogState<PopupContent>,
    filepicker_state: FileExplorerState,
    keybind_footer: FooterBuilder,
}

impl SelectTemplatePopup {
    pub fn new(template_dir: PathBuf) -> Self {
        Self {
            dialog_state: DialogState::new(PopupContent::default()),
            filepicker_state: FileExplorerState::new(template_dir),
            keybind_footer: FooterBuilder::new()
                .with_keybind(
                    FileExplorerMode::Browse,
                    "↑↓/jk".to_string(),
                    ":Move".to_string(),
                )
                .with_keybind(
                    FileExplorerMode::Browse,
                    "-".to_string(),
                    ":Go Up".to_string(),
                )
                .with_keybind(
                    FileExplorerMode::Browse,
                    ".".to_string(),
                    ":Hidden".to_string(),
                )
                .with_keybind(
                    FileExplorerMode::Browse,
                    "Enter".to_string(),
                    ":Select".to_string(),
                ),
        }
    }

    fn render_dialog(&mut self, f: &mut Frame, area: Rect) {
        let config = DialogConfig::new("Select a template to apply - Exit with `q`")
            .max_size(area.width, area.height)
            .no_buttons();

        let mut dialog =
            PopupDialog::new(&config, &mut self.dialog_state, |frame, area, _content| {
                let visible_height = area.height.saturating_sub(2) as usize;
                self.filepicker_state.ensure_visible(visible_height);

                let picker = FileExplorer::new(&self.filepicker_state)
                    .footer_builder(self.keybind_footer.clone());
                frame.render_widget(picker, area);
            });
        dialog.render(f);
    }

    fn handle_filepicker_key(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.filepicker_state.cursor_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.filepicker_state.cursor_down();
            }
            KeyCode::Enter => {
                if let Some(entry) = self.filepicker_state.current_entry() {
                    let path = entry.path.clone();
                    if path.is_file() {
                        return Action::SelectTemplate {
                            template: path,
                            project: None,
                        };
                    } else {
                        self.filepicker_state.enter_directory(path);
                    }
                }
            }
            KeyCode::Char('-') | KeyCode::Backspace => {
                self.filepicker_state.go_up();
            }
            // TODO: Support search eventually
            // KeyCode::Char('/') => {
            //     self.filepicker_state.start_search();
            // }
            KeyCode::Char('.') => {
                self.filepicker_state.toggle_hidden();
            }
            KeyCode::Char('g') => {
                self.filepicker_state.cursor_index = 0;
                self.filepicker_state.scroll = 0;
            }
            KeyCode::Char('G') => {
                let count = self.filepicker_state.visible_count();
                if count > 0 {
                    self.filepicker_state.cursor_index = count - 1;
                }
            }
            _ => {}
        }
        Action::Noop
    }
}

impl Component for SelectTemplatePopup {
    fn init(&mut self) -> Result<(), ComponentCreationError> {
        let _ = self.filepicker_state.load_entries();
        self.dialog_state.register_child(0);
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
        // let focus_target = self.dialog_state.current_focus().cloned();
        match key.code {
            // KeyCode::Tab => {
            //     if let Some(DialogFocusTarget::Child(_)) = focus_target {
            //         self.dialog_state.focus.set(DialogFocusTarget::Button(0));
            //         return Action::Noop;
            //     }
            //     if let Some(DialogFocusTarget::Button(0)) = focus_target {
            //         self.dialog_state.focus.set(DialogFocusTarget::Button(1));
            //         return Action::Noop;
            //     }
            //     if let Some(DialogFocusTarget::Button(1)) = focus_target {
            //         self.dialog_state.focus.set(DialogFocusTarget::Button(0));
            //         return Action::Noop;
            //     }
            //     return Action::Noop;
            // }
            KeyCode::Char('q') => Action::ClosePopup,
            _ => {
                // handle_scrollable_content_key(&mut self.content_state, &key, 1);
                self.handle_filepicker_key(key)
                // Action::Noop
            }
        }
    }

    fn handle_mouse_events(&mut self, _: MouseEvent) -> Action {
        // TODO: Support mouse stuff eventually
        return Action::Noop;
    }

    fn update(&mut self, _: Action) -> Action {
        return Action::Noop;
    }

    fn render(&mut self, f: &mut Frame, rect: Rect) {
        self.render_dialog(f, rect);
    }
}

#[derive(Default)]
struct PopupContent {}

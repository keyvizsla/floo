use crate::action::Action;
use crate::components::component::{Component, ComponentCreationError};
use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui_interact::components::{DialogConfig, DialogState};
use ratatui_interact::prelude::{
    PopupDialog, ScrollableContent, ScrollableContentState, handle_scrollable_content_key,
    handle_scrollable_content_mouse,
};

pub struct HelpPopup {
    dialog_state: DialogState<PopupContent>,
    content_state: ScrollableContentState,
    rect: Option<Rect>,
}

impl HelpPopup {
    pub fn new() -> Self {
        Self {
            dialog_state: DialogState::new(PopupContent::default()),
            content_state: ScrollableContentState::new(vec![
                "j / ArrowDown  Move cursor to next fireplace".to_string(),
                "k / ArrowUp    Move cursor to previous fireplace".to_string(),
                "<Enter>        Select fireplace (close floo and move to selected workspace)"
                    .to_string(),
                "n / %          Create a new fireplace".to_string(),
                "d              Delete a fireplace".to_string(),
                "h              Display help menu".to_string(),
                "q              Quit floo".to_string(),
            ]),
            rect: None,
        }
    }

    fn render_dialog(&mut self, f: &mut Frame, area: Rect) {
        let config = DialogConfig::new("Help - Exit with `q`")
            .max_size(area.width, self.height(area))
            .no_buttons();
        let mut dialog =
            PopupDialog::new(&config, &mut self.dialog_state, |frame, area, _content| {
                let text_elem = ScrollableContent::new(&self.content_state);
                frame.render_widget(text_elem, area);
            });
        dialog.render(f);
    }

    fn height(&self, area: Rect) -> u16 {
        let content_height: u16 = (self.content_state.line_count() + 4).try_into().unwrap();
        if area.height < content_height {
            area.height
        } else {
            content_height
        }
    }
}

impl Component for HelpPopup {
    fn init(&mut self) -> Result<(), ComponentCreationError> {
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
        match key.code {
            KeyCode::Char('q') => Action::ClosePopup,
            _ => {
                handle_scrollable_content_key(&mut self.content_state, &key, 1);
                Action::Noop
            }
        }
    }

    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action {
        if let Some(rect) = self.rect {
            handle_scrollable_content_mouse(&mut self.content_state, &mouse, rect, 1);
        }
        return Action::Noop;
    }

    fn update(&mut self, _action: Action) -> Action {
        return Action::Noop;
    }

    fn render(&mut self, f: &mut Frame, rect: Rect) {
        self.rect = Some(rect);
        self.render_dialog(f, rect);
    }
}

#[derive(Default)]
struct PopupContent {}

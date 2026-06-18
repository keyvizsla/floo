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

use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::layout::Rect;

use crate::action::Action;

pub struct ComponentCreationError {}

pub fn should_handle_key_event(key: &KeyEvent) -> bool {
    matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat)
}

pub trait Component {
    fn init(&mut self) -> Result<(), ComponentCreationError>;
    fn handle_events(&mut self, event: Option<Event>) -> Action {
        match event {
            Some(Event::Key(key)) if should_handle_key_event(&key) => self.handle_key_events(key),
            Some(Event::Mouse(mouse)) => self.handle_mouse_events(mouse),
            _ => Action::Noop,
        }
    }
    fn handle_key_events(&mut self, key: KeyEvent) -> Action;
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action;
    fn update(&mut self, action: Action) -> Action;
    fn render(&mut self, f: &mut Frame, rect: Rect);
}

#[cfg(test)]
mod tests {
    use super::{Component, ComponentCreationError, should_handle_key_event};
    use crate::action::Action;
    use ratatui::Frame;
    use ratatui::crossterm::event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton,
        MouseEvent, MouseEventKind,
    };
    use ratatui::layout::Rect;

    #[derive(Default)]
    struct TestComponent {
        key_events: usize,
        mouse_events: usize,
    }

    impl Component for TestComponent {
        fn init(&mut self) -> Result<(), ComponentCreationError> {
            Ok(())
        }

        fn handle_key_events(&mut self, _key: KeyEvent) -> Action {
            self.key_events += 1;
            Action::Quit
        }

        fn handle_mouse_events(&mut self, _mouse: MouseEvent) -> Action {
            self.mouse_events += 1;
            Action::ClosePopup
        }

        fn update(&mut self, _action: Action) -> Action {
            Action::Noop
        }

        fn render(&mut self, _f: &mut Frame, _rect: Rect) {}
    }

    fn key_event(kind: KeyEventKind) -> KeyEvent {
        KeyEvent {
            code: KeyCode::Char('x'),
            modifiers: KeyModifiers::NONE,
            kind,
            state: KeyEventState::NONE,
        }
    }

    fn mouse_event() -> MouseEvent {
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 0,
            row: 0,
            modifiers: KeyModifiers::NONE,
        }
    }

    #[test]
    fn ignores_release_events() {
        assert!(!should_handle_key_event(&key_event(KeyEventKind::Release)));
    }

    #[test]
    fn keeps_press_and_repeat_events() {
        assert!(should_handle_key_event(&key_event(KeyEventKind::Press)));
        assert!(should_handle_key_event(&key_event(KeyEventKind::Repeat)));
    }

    #[test]
    fn default_handler_ignores_release_events() {
        let mut component = TestComponent::default();

        assert!(matches!(
            component.handle_events(Some(Event::Key(key_event(KeyEventKind::Release)))),
            Action::Noop
        ));
        assert_eq!(component.key_events, 0);
    }

    #[test]
    fn default_handler_dispatches_press_repeat_and_mouse_events() {
        let mut component = TestComponent::default();

        assert!(matches!(
            component.handle_events(Some(Event::Key(key_event(KeyEventKind::Press)))),
            Action::Quit
        ));
        assert!(matches!(
            component.handle_events(Some(Event::Key(key_event(KeyEventKind::Repeat)))),
            Action::Quit
        ));
        assert!(matches!(
            component.handle_events(Some(Event::Mouse(mouse_event()))),
            Action::ClosePopup
        ));
        assert_eq!(component.key_events, 2);
        assert_eq!(component.mouse_events, 1);
    }
}

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

use ratatui::crossterm::event::{Event, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::layout::Rect;
use ratatui::Frame;

use crate::action::Action;

pub struct ComponentCreationError {}

pub fn handle_component_event(component: &mut dyn Component, event: Option<Event>) -> Action {
    match event {
        Some(Event::Key(key)) if should_handle_key_event(&key) => component.handle_key_events(key),
        Some(Event::Mouse(mouse)) => component.handle_mouse_events(mouse),
        _ => Action::Noop,
    }
}

pub fn should_handle_key_event(key: &KeyEvent) -> bool {
    matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat)
}

pub trait Component {
    fn init(&mut self) -> Result<(), ComponentCreationError>;
    fn handle_events(&mut self, event: Option<Event>) -> Action;
    fn handle_key_events(&mut self, key: KeyEvent) -> Action;
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action;
    fn update(&mut self, action: Action) -> Action;
    fn render(&mut self, f: &mut Frame, rect: Rect);
}

#[cfg(test)]
mod tests {
    use super::should_handle_key_event;
    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    fn key_event(kind: KeyEventKind) -> KeyEvent {
        KeyEvent {
            code: KeyCode::Char('x'),
            modifiers: KeyModifiers::NONE,
            kind,
            state: KeyEventState::NONE,
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
}

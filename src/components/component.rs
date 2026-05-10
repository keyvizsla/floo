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
use ratatui::crossterm::event::{Event, KeyEvent, MouseEvent};
use ratatui::layout::Rect;

use crate::action::Action;

pub struct ComponentCreationError {}

pub trait Component {
    fn init(&mut self) -> Result<(), ComponentCreationError>;
    fn handle_events(&mut self, event: Option<Event>) -> Action;
    fn handle_key_events(&mut self, key: KeyEvent) -> Action;
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action;
    fn update(&mut self, action: Action) -> Action;
    fn render(&mut self, f: &mut Frame, rect: Rect);
}

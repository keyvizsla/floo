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

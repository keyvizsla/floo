use crossterm::event::{Event, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::action::Action;
use crate::components::component::{Component, ComponentCreationError};
use crate::project::Project;

pub struct MainScreen {
    projects: Vec<Project>,
}

impl MainScreen {
    pub fn init_with_projects(projects: Vec<Project>) -> Self {
        MainScreen { projects }
    }
}

impl Component for MainScreen {
    fn init(&mut self) -> Result<(), ComponentCreationError> {
        todo!()
    }

    fn handle_events(&mut self, event: Option<Event>) -> Action {
        todo!()
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        todo!()
    }

    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action {
        todo!()
    }

    fn update(&mut self, action: Action) -> Action {
        todo!()
    }

    fn render(&mut self, f: &mut Frame, rect: Rect) {
        todo!()
    }
}
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

use crossterm::event::{Event, KeyEvent, MouseEvent};
use ratatui::{Frame, layout::Rect};

use crate::{
    action::Action,
    components::{
        component::{Component, ComponentCreationError},
        main_screen::MainScreen,
        start_screen::StartScreen,
    },
    fireplace::Fireplace,
    utils::{remove_project, replace_project},
};

#[derive(Default)]
pub struct Tui {
    projects: Vec<Fireplace>,
    start_screen: StartScreen,
    main_screen: MainScreen,
}

impl Tui {
    pub fn new(projects: Vec<Fireplace>) -> Self {
        Self {
            projects,
            ..Default::default()
        }
    }
}

impl Component for Tui {
    fn init(&mut self) -> Result<(), ComponentCreationError> {
        self.main_screen = MainScreen::init_with_projects(self.projects.clone());
        self.main_screen.init()?;
        self.start_screen.init()?;
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Action {
        // For now the base tui has to do no own handling
        // it just passes events up to the base app
        if self.projects.len() > 0 {
            self.main_screen.handle_events(event)
        } else {
            self.start_screen.handle_events(event)
        }
    }

    fn handle_key_events(&mut self, _: KeyEvent) -> Action {
        // This should never be called normally
        Action::Noop
    }

    fn handle_mouse_events(&mut self, _: MouseEvent) -> crate::action::Action {
        // This should never be called normally
        Action::Noop
    }

    fn update(&mut self, action: Action) -> Action {
        match action.clone() {
            Action::AddFireplace(project) => self.projects.push(project),
            Action::DeleteFireplace(project) => remove_project(&mut self.projects, &project),
            Action::ReplaceFireplace {
                old: old_project,
                new: new_project,
            } => replace_project(&mut self.projects, &old_project, new_project),
            _ => {}
        }
        if self.projects.len() > 0 {
            self.main_screen.update(action.clone());
        } else {
            self.start_screen.update(action);
        }
        Action::Noop
    }

    fn render(&mut self, f: &mut Frame, rect: Rect) {
        if self.projects.len() > 0 {
            self.main_screen.render(f, rect);
        } else {
            self.start_screen.render(f, rect);
        }
    }
}

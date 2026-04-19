use ratatui::widgets::ListState;

use crate::action::Action;
use crate::project::Project;
use crossterm::event::{self, Event, KeyCode};
use std::io::{self, Stdout};

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub projects: Vec<Project>,
    selected_index: usize,
    pub project_list_state: ListState,
}

impl AppState {
    pub fn new(projects: Vec<Project>) -> Self {
        Self {
            projects,
            selected_index: 0,
            project_list_state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn selected_project(&self) -> Option<&Project> {
        return self.projects.get(self.selected_index);
    }

    pub fn handle_input(&mut self) -> Result<Option<Action>, io::Error> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => Ok(Some(Action::Quit)),
                KeyCode::Char('j') | KeyCode::Down => {
                    self.select_next();
                    return Ok(None);
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.select_prev();
                    return Ok(None);
                }
                KeyCode::Enter => Ok(Some(Action::Pick(
                    self.projects[self.selected_index].clone(),
                ))),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    fn select_next(&mut self) {
        self.selected_index += 1;
        if self.selected_index >= self.projects.len() {
            self.selected_index = 0;
        }
        self.project_list_state.select(Some(self.selected_index));
    }

    fn select_prev(&mut self) {
        if self.selected_index == 0 {
            self.selected_index = self.projects.len() - 1;
        } else {
            self.selected_index -= 1;
        }
        self.project_list_state.select(Some(self.selected_index));
    }
}

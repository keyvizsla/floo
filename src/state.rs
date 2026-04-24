use ratatui::widgets::ListState;

use crate::action::Action;
use crate::project::Project;
use crossterm::event::{self, Event, KeyCode};
use std::io::{self};
use std::process::exit;
use crate::db_handler::get_projects;

/// Represents the global state of the App
#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub projects: Vec<Project>,
}

impl AppState {
    pub fn new(projects: Vec<Project>) -> Self {
        Self {
            projects,
        }
    }
    pub fn init() -> Self {
        let projects = match get_projects() {
            Ok(projects) => projects,
            Err(_) => {
                println!("Unable to access or create database. Please report this error.");
                exit(1);
            }
        };

        Self {
            projects
        }
    }
}

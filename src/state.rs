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

use crate::db_handler::get_projects;
use crate::fireplace::Fireplace;
use crate::utils::replace_project;
use std::process::exit;

// TODO: This is not really needed anymore, we should get rid of it.

/// Represents the global state of the App
#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub projects: Vec<Fireplace>,
}

impl AppState {
    pub fn init() -> Self {
        let projects = match get_projects() {
            Ok(projects) => projects,
            Err(_) => {
                println!("Unable to access or create database. Please report this error.");
                exit(1);
            }
        };

        Self { projects }
    }

    pub fn remove_project(&mut self, project: &Fireplace) {
        self.projects.retain(|p| p.name != project.name);
    }

    pub fn replace_project(&mut self, old_project: &Fireplace, new_project: Fireplace) {
        replace_project(&mut self.projects, old_project, new_project);
    }
}

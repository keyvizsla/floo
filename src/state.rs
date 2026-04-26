use crate::db_handler::get_projects;
use crate::project::Project;
use std::process::exit;

/// Represents the global state of the App
#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub projects: Vec<Project>,
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

    pub fn remove_project(&mut self, project: &Project) {
        self.projects.retain(|p| p.name != project.name);
    }
}

use crate::db_handler::get_projects;
use crate::project::Project;
use crate::utils::replace_project;
use std::process::exit;

// TODO: This is not really needed anymore, we should get rid of it.

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

    pub fn replace_project(&mut self, old_project: &Project, new_project: Project) {
        replace_project(&mut self.projects, old_project, new_project);
    }
}

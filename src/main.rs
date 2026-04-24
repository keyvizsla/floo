use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    env,
    io::{self, Stdout},
    path::PathBuf,
    process::exit,
};

use crate::db_handler::get_projects;
// use crate::layout::draw;
use crate::{action::Action, project::Project, state::AppState};
use crate::app::App;

mod action;
mod components;
mod db_handler;
// mod layout;
mod project;
mod state;
mod app;

/// Initialize the TUI-App and return the Terminal object.

fn init_projects() -> Vec<Project> {
    match get_projects() {
        Ok(projects) => projects,
        Err(_) => {
            println!("Unable to access or create database. Please report this error.");
            exit(1);
        }
    }
}

fn output_shell_cmd(project: &Project, output_path: &PathBuf) -> Result<(), io::Error> {
    let mut instructions = String::new();

    instructions.push_str(&format!("cd '{}'\n", project.directory.to_str().unwrap()));

    let project_script_path = project.directory.join(".floo");
    if project_script_path.exists() {
        instructions.push_str(&format!("source '{}'\n", project_script_path.display()));
    }

    std::fs::write(output_path, instructions)?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let mut app = App::new().map_err(|_| io::Error::from(io::ErrorKind::Other))?;
    app.run();
    Ok(())
}

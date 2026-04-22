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
use crate::layout::draw;
use crate::{action::Action, project::Project, state::AppState};

mod action;
mod db_handler;
mod layout;
mod project;
mod state;

/// Initialize the TUI-App and return the Terminal object.
fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

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
    let mut terminal = init_terminal()?;
    let items = init_projects();

    let output_path = env::var("FLOO_OUTPUT_FILE")
        .ok()
        .map(PathBuf::from)
        .unwrap();

    let mut state = AppState::new(items);

    loop {
        terminal.draw(|frame| draw(frame, &mut state))?;

        match state.handle_input()? {
            Some(Action::Quit) => {
                break;
            }
            Some(Action::Pick(proj)) => {
                let _ = output_shell_cmd(&proj, &output_path);
                break;
            }
            _ => {}
        }
    }

    // 4. Cleanup Terminal (Crucial for a usable terminal after exit)
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

use crate::action::Action;
use crate::components::component::Component;
use crate::components::tui::Tui;
use crate::db_handler;
use crate::errors::FlooError;
use crate::project::Project;
use crate::state::AppState;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{event, execute};
use edit::Builder;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Stdout, stdout};
use std::path::PathBuf;
use std::{env, fs};

use ratatui::crossterm::ExecutableCommand;

pub struct AppCreationError {}
pub struct App {
    state: AppState,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    tui: Tui,
}

impl App {
    fn output_path() -> PathBuf {
        env::var("FLOO_OUTPUT_FILE")
            .ok()
            .map(PathBuf::from)
            .unwrap()
    }

    fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        Ok(Terminal::new(backend)?)
    }

    fn edit_notes(&mut self, notes: String) -> Result<String, io::Error> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        let mut binding = Builder::new();
        let tempfile = binding.suffix(".md");
        let edited = edit::edit_with_builder(notes, tempfile)?;
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        self.terminal.clear()?;
        Ok(edited)
    }

    fn edit_and_apply_template(
        &mut self,
        template: &PathBuf,
        project: &Project,
    ) -> Result<(), io::Error> {
        let file_contents = String::from_utf8(fs::read(template)?)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file contents"))?;
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        let mut binding = Builder::new();
        let tempfile = binding.suffix(".sh");
        let edited = edit::edit_with_builder(file_contents, tempfile)?;
        fs::write(project.directory.join(".floo"), edited)?;
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn new() -> Result<Self, AppCreationError> {
        let state = AppState::init();
        let terminal = Self::init_terminal().map_err(|_| AppCreationError {})?;
        let mut tui = Tui::new(state.projects.clone());
        let _ = tui.init();
        Ok(App {
            state,
            terminal,
            tui,
        })
    }

    fn output_shell_cmd(project: &Project, output_path: &PathBuf) -> Result<(), io::Error> {
        let mut instructions = String::new();

        let project_script_path = project.directory.join(".floo");
        if project_script_path.exists() {
            instructions.push_str(&format!("source '{}'\n", project_script_path.display()));
        } else {
            instructions.push_str(&format!("cd '{}'\n", project.directory.to_str().unwrap()));
        }

        std::fs::write(output_path, instructions)?;
        Ok(())
    }

    fn draw(&mut self) {
        let _ = self
            .terminal
            .draw(|frame| self.tui.render(frame, frame.area()));
    }

    fn cleanup(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }

    /// Start the TUI with an open, prefilled fireplace
    /// creation popup. This should be called when `floo create`
    /// is ran.
    pub fn run_with_prefilled_popup(
        &mut self,
        prefill: Option<Project>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.tui.update(Action::OpenCreationPopup(prefill));
        self.run()
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            self.draw();
            if event::poll(std::time::Duration::from_millis(100))? {
                let action = self.tui.handle_events(event::read().ok());
                let tui_update = match action.clone() {
                    Action::EditNotes(project) => {
                        if let Ok(updated_notes) = self.edit_notes(project.notes.clone()) {
                            let new_project = Project {
                                name: project.name.clone(),
                                directory: project.directory.clone(),
                                notes: updated_notes.clone(),
                                last_accessed: project.last_accessed.clone(),
                            };
                            match db_handler::change_notes(&project, &updated_notes) {
                                Ok(_) => {
                                    self.state.remove_project(&project);
                                    self.state.replace_project(&project, new_project.clone());
                                    Action::ReplaceProject {
                                        old: project,
                                        new: new_project,
                                    }
                                }
                                Err(_) => Action::Error(FlooError::DbUpdateError(
                                    "Failed to update notes".to_string(),
                                )),
                            }
                        } else {
                            Action::Noop
                        }
                    }
                    Action::Quit => {
                        self.cleanup();
                        return Ok(());
                    }
                    Action::Pick(project) => {
                        // Ignore errors in the database update, since these are not critical
                        let _ = db_handler::set_last_accessed_to_now(&project);
                        let _ = Self::output_shell_cmd(&project, &Self::output_path());
                        self.cleanup();
                        return Ok(());
                    }
                    Action::AddFireplace(project) => match db_handler::add_project(project.clone())
                    {
                        Ok(_) => {
                            self.state.projects.push(project.clone());
                            action
                        }
                        Err(_) => Action::Error(FlooError::DbUpdateError(
                            "Could not add fireplace.".to_string(),
                        )),
                    },
                    Action::DeleteFireplace(project) => {
                        match db_handler::remove_project(project.clone()) {
                            Ok(_) => {
                                self.state.remove_project(&project);
                                action
                            }
                            Err(_) => Action::Error(FlooError::DbUpdateError(
                                "Could not delete fireplace.".to_string(),
                            )),
                        }
                    }
                    Action::SelectTemplate { template, project } => {
                        if let Some(project) = project {
                            // TODO: Handle errors
                            let _ = self.edit_and_apply_template(&template, &project);
                        }
                        Action::Noop
                    }
                    _ => Action::Noop,
                };
                self.tui.update(tui_update);
            }
        }
    }
}

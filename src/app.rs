use std::env;
use crate::state::AppState;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
};
use std::io::{self, Stdout};
use std::path::PathBuf;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crate::action::Action;
use crate::components::component::Component;
use crate::components::main_screen::MainScreen;
use crate::components::start_screen::StartScreen;
use crate::db_handler;

pub struct AppCreationError {}
pub struct App {
    state: AppState,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    start_screen: StartScreen,
    main_screen: MainScreen,
}

impl App {
    fn output_path() {
        let output_path = env::var("FLOO_OUTPUT_FILE")
            .ok()
            .map(PathBuf::from)
            .unwrap();

    }
    fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        Ok(Terminal::new(backend)?)
    }
    pub fn new() -> Result<Self, AppCreationError> {
        let state = AppState::init();
        let terminal = Self::init_terminal().map_err(|_| AppCreationError {})?;
        let mut start_screen = StartScreen::default();
        let _ = start_screen.init();
        let main_screen = MainScreen::init_with_projects(state.projects.clone());
        Ok(App { state, terminal, start_screen, main_screen })
    }

    fn draw(&mut self) {
        let _ = self.terminal.draw(|frame| {
            if self.state.projects.len() > 0 {
                self.main_screen.render(frame, frame.area());
            } else {
                self.start_screen.render(frame, frame.area());
            }
        });
    }

    // TODO: This kind of motivates having a base tui component instead of rendering in App
    fn handle_events(&mut self) -> Action {
        let action = if self.state.projects.len() > 0 {
            self.main_screen.handle_events(event::read().ok())
        } else {
            self.start_screen.handle_events(event::read().ok())
        };

        match action {
            Action::AddFireplace(project) => {
                // TODO: write project to database, not just in memory
                let _ = db_handler::add_project(project.clone());
                self.state.projects.push(project.clone());
                self.main_screen.add_project(project);
                Action::Noop
            },
           Action::Quit => {
                self.cleanup();
               return Action::Quit;
            },
            _ => Action::Noop,
        }
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

    pub fn run(&mut self) {
        loop {
            self.draw();
            let action = self.handle_events();
            if let Action::Quit = action {
                break;
            }
        }
        self.cleanup();
    }
}

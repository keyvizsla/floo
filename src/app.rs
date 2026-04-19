use crate::state::AppState;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::io::{self, Stdout};

pub struct App {
    state: AppState,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    output_path: String,
}

impl App {}

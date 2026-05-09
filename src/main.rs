use clap::{Parser, Subcommand};
use std::{
    io::{self},
    path::PathBuf,
};

use crate::{app::App, project::Project, utils::init_sys};

mod action;
mod app;
mod components;
mod db_handler;
mod errors;
mod project;
mod state;
mod utils;

#[derive(Parser)]
#[command(name = "FLOO")]
#[command(version)]
#[command(about = "TODO: Write about section", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Init,
    Create {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Command::Init) => {
            init_sys();
            Ok(())
        }
        Some(Command::Create { path }) => {
            let mut app = App::new().map_err(|_| io::Error::from(io::ErrorKind::Other))?;
            let mut prefill = Project::default();
            prefill.directory = path.canonicalize()?;
            app.run_with_prefilled_popup(Some(prefill))
                .map_err(|_| io::Error::from(io::ErrorKind::Other))
        }
        None => {
            let mut app = App::new().map_err(|_| io::Error::from(io::ErrorKind::Other))?;
            app.run().map_err(|_| io::Error::from(io::ErrorKind::Other))
        }
    }
}

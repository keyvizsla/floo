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
#[command(
    name = "floo",
    bin_name = "floo",
    version,
    author = "Leon Degel-Koehn <leon.koehn2002@gmail.com>",
    about = "Effortless travel between and to workspaces",
    long_about = "floo v0.1.0-beta.1\n\nEffortless travel between and to workspaces\n\nSee also: <link here>",
    long_version = "v0.1.0-beta.1\nAuthor: Leon Degel-Koehn <leon.koehn2002@gmail.com>\nCopyright (c) 2026"
)]
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

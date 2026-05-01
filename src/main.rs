use clap::{Parser, Subcommand};
use std::io::{self};

use crate::{app::App, utils::init_sys};

mod action;
mod app;
mod components;
mod db_handler;
mod project;
mod state;
mod utils;

#[derive(Parser)]
#[command(name = "mytool")]
#[command(about = "A versatile CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Init,
}

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Command::Init) => {
            init_sys();
            Ok(())
        }
        None => {
            let mut app = App::new().map_err(|_| io::Error::from(io::ErrorKind::Other))?;
            app.run();
            Ok(())
        }
    }
}

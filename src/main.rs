/*
 * Copyright (C) 2026 Leon Degel-Koehn
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::io::{self};

use clap::Parser;

use crate::{
    app::App,
    cli::{Cli, Command},
    project::Project,
    utils::init_sys,
};

mod action;
mod app;
mod cli;
mod components;
mod db_handler;
mod errors;
mod project;
mod state;
mod utils;

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
        Some(Command::Template { filepath, name }) => {
            todo!()
        }
        None => {
            let mut app = App::new().map_err(|_| io::Error::from(io::ErrorKind::Other))?;
            app.run().map_err(|_| io::Error::from(io::ErrorKind::Other))
        }
    }
}

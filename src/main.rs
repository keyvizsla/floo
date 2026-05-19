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
    fireplace::Fireplace,
    utils::{edit_and_save_template, init_sys, install_default_templates, install_local_templates},
};

mod action;
mod app;
mod cli;
mod components;
mod db_handler;
mod errors;
mod fireplace;
mod shell;
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
            let mut prefill = Fireplace::default();
            let _ = prefill.set_directory(path.clone());
            app.run_with_prefilled_popup(Some(prefill))
                .map_err(|_| io::Error::from(io::ErrorKind::Other))
        }
        Some(Command::Template { filepath, name }) => {
            let res = edit_and_save_template(filepath.to_path_buf(), name.to_string());
            match res {
                Ok(_) => println!("Successfully saved your new template."),
                Err(_) => eprintln!("Failed to save you new template."),
            };
            res
        }
        Some(Command::InstallTemplates { local }) => {
            if let Some(template_path) = local {
                install_local_templates(template_path);
            } else {
                install_default_templates();
            }
            println!("Successfully installed all default floo templates.");
            Ok(())
        }
        None => {
            let mut app = App::new().map_err(|_| io::Error::from(io::ErrorKind::Other))?;
            app.run().map_err(|_| io::Error::from(io::ErrorKind::Other))
        }
    }
}

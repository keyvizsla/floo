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

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "floo",
    bin_name = "floo",
    version,
    author = "Leon Degel-Koehn <leon.koehn2002@gmail.com>",
    about = "Effortless travel between and to workspaces",
    after_help = "Project homepage: <github pages link>\nRepository: https://github.com/Leon-Degel-Koehn/floo",
    help_template = "{before-help}{name} {version}{author-section}{about-section}\n{usage-heading} {usage}\n\n{all-args}{after-help}"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(hide = true)]
    Init,

    /// Open floo with a prefilled fireplace create popup containing the specified path
    Create {
        /// Path to the directory for which to set up a fireplace
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

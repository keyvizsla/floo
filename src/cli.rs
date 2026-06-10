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

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use crate::shell::{BashBackend, NuBackend, ShellBackend, ZshBackend};

#[derive(Parser)]
#[command(
    name = "floo",
    bin_name = "floo",
    version,
    author = "Leon Degel-Koehn <leon.koehn2002@gmail.com>",
    about = "Effortless travel to and from workspaces",
    after_help = "Homepage: https://keyvizsla.github.io/floo\nRepository: https://github.com/keyvizsla/floo",
    help_template = "{before-help}{name} {version}{author-section}{about-section}\n{usage-heading} {usage}\n\n{all-args}{after-help}"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
    #[arg(long, value_enum, default_value = "bash")]
    pub shell: Shell,
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

    /// Create a new template from a given shell script
    Template {
        /// Path to the file to base the template on
        filepath: PathBuf,

        /// Name of the template
        name: String,
    },

    /// Install floo templates (will override local templates of the same name unless --local is used)
    InstallTemplates {
        /// Install templates from a local directory instead of remote (e.g. .../floo/templates)
        #[arg(long, value_name = "PATH")]
        local: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Nu,
}

impl Shell {
    pub fn get_backend(&self) -> Box<dyn ShellBackend> {
        match self {
            Self::Bash => Box::new(BashBackend),
            Self::Zsh => Box::new(ZshBackend),
            Self::Nu => Box::new(NuBackend),
        }
    }
}

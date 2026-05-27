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

use std::fmt::Display;

#[derive(Clone)]
pub enum FlooError {
    DbUpdateError(String),
    AppDataDirError(String),
    NoTemplates,
}

impl Display for FlooError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base_error_message = "Please report this issue including some information about your system and the output of floo --version to: \
            https://github.com/keyvizsla/floo/issues/new.";
        match self {
            Self::AppDataDirError(msg) | Self::DbUpdateError(msg) => write!(
                f,
                "{} \
                 {}",
                msg, base_error_message
            ),
            _ => write!(
                f,
                "Floo encountered an unexpected error. \
                 {}",
                base_error_message
            ),
        }
    }
}

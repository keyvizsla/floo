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

use crate::{errors::FlooError, project::Project};
use std::path::PathBuf;

#[derive(Clone)]
pub enum Action {
    Quit,
    AddFireplace(Project),
    DeleteFireplace(Project),
    Pick(Project),
    OpenCreationPopup(Option<Project>),
    ClosePopup,
    EditNotes(Project),
    ReplaceProject {
        old: Project,
        new: Project,
    },
    SelectTemplate {
        template: PathBuf,
        project: Option<Project>,
    },
    Error(FlooError),
    Noop,
}

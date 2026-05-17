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

use crate::{errors::FlooError, fireplace::Fireplace};
use std::path::PathBuf;

#[derive(Clone)]
pub enum Action {
    Quit,
    AddFireplace(Fireplace),
    DeleteFireplace(Fireplace),
    Pick(Fireplace),
    OpenCreationPopup(Option<Fireplace>),
    ClosePopup,
    EditNotes(Fireplace),
    ReplaceFireplace {
        old: Fireplace,
        new: Fireplace,
    },
    SelectTemplate {
        template: PathBuf,
        project: Option<Fireplace>,
    },
    Error(FlooError),
    Noop,
}

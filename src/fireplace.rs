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

use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug, Default, Clone)]
pub struct Fireplace {
    pub name: String,
    directory: PathBuf,
    pub notes: String,
    pub last_accessed: i64,
}

impl Fireplace {
    /// Create a new fireplace with specific attributes
    pub fn new(name: String, directory: PathBuf, notes: String, last_accessed: i64) -> Self {
        let mut result = Fireplace {
            name,
            directory: PathBuf::default(),
            notes,
            last_accessed,
        };
        let _ = result.set_directory(directory);
        result
    }

    /// Return the true path to the readme file of the project.
    /// This handles different common spellings for the readme file.
    fn get_description_path(&self) -> Option<PathBuf> {
        let supported_variants = [
            "README.md",
            "README",
            "Readme",
            "Readme.md",
            "readme",
            "readme.md",
        ];

        for supported_variant in supported_variants {
            let readme_path = self.directory.join(supported_variant);
            match fs::exists(&readme_path) {
                Ok(true) => {
                    return Some(readme_path);
                }
                _ => {}
            }
        }
        None
    }

    /// Get the description(readme contents) of the given project
    /// and return them. If no such description can be found, then
    /// None is returned.
    pub fn get_description(&self) -> Option<String> {
        let description_filepath = match self.get_description_path() {
            Some(path) => path,
            _ => {
                return None;
            }
        };

        let description = match fs::read_to_string(description_filepath) {
            Ok(content) => content,
            _ => {
                return None;
            }
        };

        Some(description)
    }

    /// Getter for the Fireplace's root directory path
    pub fn get_directory(&self) -> PathBuf {
        self.directory.clone()
    }

    /// Setter for the Fireplace's root directory path
    pub fn set_directory(&mut self, dir: PathBuf) -> io::Result<()> {
        let dir_str = dir.to_str();
        if dir_str.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidFilename,
                "Encountered invalid UTF-8",
            ));
        }
        let expanded_path = shellexpand::full(dir_str.unwrap()).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unable to expand some expressions",
            )
        })?;
        self.directory = fs::canonicalize(expanded_path.as_ref())?;
        Ok(())
    }

    /// Returns true iff the fireplace has an associated .floo script
    pub fn has_startup_script(&self) -> bool {
        self.directory.join(".floo").exists()
    }
}

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

use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use directories::ProjectDirs;
use edit::Builder;
use ratatui::{Terminal, prelude::CrosstermBackend};
use reqwest::blocking::get;
use serde::Deserialize;
use std::path::Path;

use crate::errors::FlooError;
use crate::fireplace::Fireplace;
use std::{
    env, fs,
    io::{self, Stdout, stdout},
    path::PathBuf,
};

/// Remove a project from the given list of projects.
/// If the project is not contained, nothing happens.
pub fn remove_project(projects: &mut Vec<Fireplace>, project_to_delete: &Fireplace) {
    projects.retain(|p| p.name != project_to_delete.name);
}

pub fn replace_project(
    projects: &mut Vec<Fireplace>,
    old_project: &Fireplace,
    new_project: Fireplace,
) {
    for project in projects {
        if project.name == old_project.name {
            *project = new_project;
            return;
        }
    }
}

/// Outputs the floo shell wrapper function.
/// Should only be used by the installer.
pub fn init_sys() {
    let shell_wrapper = r#"
floo() {
    local tmp_file
    tmp_file="$(mktemp)"
    export FLOO_OUTPUT_FILE="$tmp_file"

    command floo-bin "$@"

    if [ -s "$tmp_file" ]; then
        . "$tmp_file"
    fi

    rm -f "$tmp_file"
    unset FLOO_OUTPUT_FILE
}
"#;

    println!("{}", shell_wrapper);
}

/// Return the length (number of characters)
/// of the longest line in the string.
pub fn longest_line(text: &str) -> usize {
    let mut max: usize = 0;
    for line in text.lines() {
        let length = line.len();
        if length > max {
            max = length;
        }
    }
    max
}

/// Return the directory path to which floo writes persistent data.
/// This function panics if the directory could not be resolved without issues.
pub fn appdata_dir() -> PathBuf {
    let override_data_dir = env::var("FLOO_APPDATA_DIR").ok().map(PathBuf::from);
    if let Some(override_path) = override_data_dir {
        return override_path;
    }
    let floo_directory = ProjectDirs::from("", "", "floo")
        .unwrap_or_else(|| {
            panic!(
                "{}",
                FlooError::AppDataDirError(
                    "Unable to determine valid appdata storage directory for floo.".to_string(),
                )
            )
        })
        .data_dir()
        .to_path_buf();
    if !floo_directory.exists() {
        std::fs::create_dir_all(&floo_directory).unwrap_or_else(|_| {
            panic!(
                "{}",
                FlooError::AppDataDirError("Unable to create floo appdata directory.".to_string())
            )
        });
    }
    floo_directory
}

/// Return the path to the default/configured
/// directory containing template startup scripts.
pub fn get_template_dir() -> PathBuf {
    let template_dir = appdata_dir().join("templates");
    if !template_dir.exists() {
        std::fs::create_dir_all(&template_dir).expect("Failed to create template directory");
    }
    template_dir
}

/// Open the editor and return the edited contents
pub fn open_editor(
    file_contents: String,
    file_suffix: Option<String>,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<String, io::Error> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    let edited = if let Some(suffix) = file_suffix {
        let mut binding = Builder::new();
        let tempfile = binding.suffix(&suffix);
        edit::edit_with_builder(file_contents, tempfile)?
    } else {
        edit::edit(file_contents)?
    };
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    terminal.clear()?;
    Ok(edited)
}

/// Read the file contents stored at the given filepath and
/// save the edited contents as a new template with the given name.
pub fn edit_and_save_template(filepath: PathBuf, template_name: String) -> Result<(), io::Error> {
    let contents = String::from_utf8(fs::read(filepath)?).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidInput, "Unable to parse file as utf-8")
    })?;
    let mut binding = Builder::new();
    let tempfile = binding.suffix(".sh");
    let edited = edit::edit_with_builder(contents, tempfile)?;
    let target_path = get_template_dir().join(template_name);
    fs::write(target_path, edited)?;
    Ok(())
}

#[derive(Deserialize)]
struct GithubFileEntry {
    name: String,
    path: String,
    #[serde(rename = "type")]
    kind: String,
    download_url: Option<String>,
}

/// Install the default .floo templates from github
pub fn install_default_templates() {
    download_github_dir(
        "https://api.github.com/repos/keyvizsla/floo/contents/templates",
        &get_template_dir(),
    )
}

fn download_github_dir(api_url: &str, dest: &Path) {
    let client = reqwest::blocking::Client::new();
    let error_msg = "Failed to download some refs.";

    let entries: Vec<GithubFileEntry> = client
        .get(api_url)
        .header("User-Agent", "reqwest")
        .send()
        .expect(error_msg)
        .json()
        .expect(error_msg);

    for entry in entries {
        match entry.kind.as_str() {
            "file" => {
                let bytes = get(entry.download_url.unwrap())
                    .expect(error_msg)
                    .bytes()
                    .expect(error_msg);

                fs::write(dest.join(entry.name), bytes).expect(error_msg);
            }

            "dir" => {
                download_github_dir(
                    &format!(
                        "https://api.github.com/repos/keyvizsla/floo/contents/{}",
                        entry.path
                    ),
                    &dest.join(entry.name),
                );
            }

            _ => {}
        }
    }
}

pub fn install_local_templates(template_dir: &PathBuf) {
    copy_dir_all(template_dir, get_template_dir()).expect("Failed to install local templates.")
}

/// Copy all contents of the src directory to the dest directory recursively.
fn copy_dir_all(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    let src = src.as_ref();
    let dst = dest.as_ref();

    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        let target_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(entry.path(), target_path)?;
        } else {
            fs::copy(entry.path(), target_path)?;
        }
    }
    Ok(())
}

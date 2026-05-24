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

use std::{
    io,
    path::{Path, PathBuf},
};

use crate::fireplace::Fireplace;

/// Output the shell script that is to be executed upon selecting the given fireplace.
/// The script is written to the given output_path.
pub fn output_shell_cmd(fireplace: &Fireplace, output_path: &PathBuf) -> io::Result<()> {
    let mut instructions = String::new();
    let project_script_path = fireplace.get_directory().join(".floo");

    if project_script_path.exists() {
        set_custom_floo_script(&mut instructions, fireplace, &project_script_path);
    } else {
        set_default_floo_script(&mut instructions, fireplace);
    }

    std::fs::write(output_path, instructions)?;
    Ok(())
}

fn set_custom_floo_script(script: &mut String, fireplace: &Fireplace, floo_script_path: &Path) {
    set_floo_env(script, fireplace);
    script.push_str(&format!("source '{}'\n", floo_script_path.display()));
}

fn set_default_floo_script(instructions: &mut String, fireplace: &Fireplace) {
    let potential_env_files = [".env", ".envrc"];
    for file in potential_env_files {
        let full_filepath = fireplace.get_directory().join(file);
        if full_filepath.is_file() {
            instructions.push_str(&format!("source '{}'\n", full_filepath.to_str().unwrap()));
        }
    }
    instructions.push_str(&format!(
        "cd '{}'\n",
        fireplace.get_directory().to_str().unwrap()
    ));
}

/// Make FLOO environment variables available to .floo scripts
fn set_floo_env(script: &mut String, fireplace: &Fireplace) {
    script.push_str(&format!(
        "FLOO_DIR='{}'\n",
        fireplace.get_directory().to_str().unwrap()
    ));
    script.push_str(&format!("FLOO_NAME='{}'\n", fireplace.name));
}

/// Check that the shell environment was correctly configured for floo.
/// Return an error if not.
pub fn check_env(verbose: bool) -> io::Result<()> {
    if std::env::var("FLOO_OUTPUT_FILE").is_err() {
        let error_str =             "Please ensure you have added `eval \"$(floo-bin init)\"` to your .bashrc or .zshrc. and you run floo as `floo` and not as `floo-bin`";
        if verbose {
            eprintln!("{}", error_str);
        }
        return Err(io::Error::other(
            "FLOO_OUTPUT_FILE not set",
        ));
    }
    Ok(())
}

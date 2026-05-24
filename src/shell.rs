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
    collections::HashMap,
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
};

use serde::Serialize;

use crate::{cli::Shell, fireplace::Fireplace};

/// Output the shell script that is to be executed upon selecting the given fireplace.
/// The script is written to the given output_path.
pub fn output_shell_cmd(
    fireplace: &Fireplace,
    output_path: &PathBuf,
    shell: Shell,
) -> io::Result<()> {
    match shell {
        Shell::Bash | Shell::Zsh => {
            let mut instructions = Vec::new();
            let project_script_path = fireplace.get_directory().join(".floo");

            if project_script_path.exists() {
                set_custom_floo_script(&mut instructions, fireplace, &project_script_path);
            } else {
                set_default_floo_script(&mut instructions, fireplace);
            }

            std::fs::write(
                output_path,
                instructions
                    .iter()
                    .map(|c| c.render(Shell::Bash))
                    .collect::<Vec<String>>()
                    .join("\n"),
            )?;
            Ok(())
        }
        Shell::Nu => {
            let filepath = fireplace.get_directory().join(".env");
            let output = NuModeOutput {
                env: HashMap::new(),
                dir: Some(fireplace.get_directory()),
            };
            if filepath.is_file() {
                let contents = fs::read_to_string(filepath)?;
                let mut map = HashMap::new();
                for item in dotenvy::from_read_iter(contents.as_bytes()) {
                    let (key, value) = item.map_err(io::Error::other)?;
                    map.insert(key, value);
                }
            }
            std::fs::write(output_path, serde_json::to_string(&output)?)?;
            Ok(())
        }
    }
}

fn set_custom_floo_script(
    script: &mut Vec<Commands>,
    fireplace: &Fireplace,
    floo_script_path: &Path,
) {
    set_floo_env(script, fireplace);
    script.push(Commands::Raw(format!(
        "source '{}'\n",
        floo_script_path.display()
    )));
}

fn set_default_floo_script(instructions: &mut Vec<Commands>, fireplace: &Fireplace) {
    let potential_env_files = [".env", ".envrc"];
    for file in potential_env_files {
        let full_filepath = fireplace.get_directory().join(file);
        if full_filepath.is_file() {
            instructions.push(Commands::Raw(format!(
                "source '{}'\n",
                full_filepath.to_str().unwrap()
            )));
        }
    }
    instructions.push(Commands::Cd(fireplace.get_directory()));
}

/// Make FLOO environment variables available to .floo scripts
fn set_floo_env(script: &mut Vec<Commands>, fireplace: &Fireplace) {
    script.push(Commands::Env {
        key: OsString::from("FLOO_DIR"),
        value: fireplace.get_directory().to_str().unwrap().into(),
    });
    script.push(Commands::Env {
        key: OsString::from("FLOO_NAME"),
        value: fireplace.name.clone().into(),
    });
}

/// Check that the shell environment was correctly configured for floo.
/// Return an error if not.
pub fn check_env(verbose: bool) -> io::Result<()> {
    if std::env::var("FLOO_OUTPUT_FILE").is_err() {
        let error_str = "Please ensure you have added `eval \"$(floo-bin init)\"` to your .bashrc or .zshrc. and you run floo as `floo` and not as `floo-bin`";
        if verbose {
            eprintln!("{}", error_str);
        }
        return Err(io::Error::other("FLOO_OUTPUT_FILE not set"));
    }
    Ok(())
}

// SHELL ABSTRACTIONS

pub enum Commands {
    Init,
    Cd(PathBuf),
    Env { key: OsString, value: OsString },
    Raw(String),
}

impl Commands {
    pub fn render(&self, shell: Shell) -> String {
        match self {
            Commands::Init => match shell {
                Shell::Bash | Shell::Zsh => r#"
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
                "#
                .to_string(),
                Shell::Nu => r#"
                    def --env --wrapped floo [...args] {
                        let tmp_file = (mktemp)
                        $env.FLOO_OUTPUT_FILE = $tmp_file

                        ^floo-bin ...$args

                        let output = (
                            if (
                                ($tmp_file | path exists)
                                and
                                ((ls $tmp_file).0.size > 0)
                            ) {
                                open $tmp_file
                            } else {
                                {}
                            }
                        )

                        # Load env vars
                        if (($output.env? | default {}) | is-not-empty) {
                            load-env $output.env
                        }

                        # Change directory
                        if (($output.cwd? | default "") | is-not-empty) {
                            cd $output.cwd
                        }

                        rm -f $tmp_file
                        hide-env FLOO_OUTPUT_FILE
                    }
                "#
                .to_string(),
            },
            Commands::Cd(path) => format!("cd '{}'", path.to_str().unwrap()),
            Commands::Env { key, value } => match shell {
                Shell::Bash | Shell::Zsh => {
                    format!("{}='{}'", key.to_str().unwrap(), value.to_str().unwrap())
                }
                Shell::Nu => format!(
                    "$env.{} = '{}'",
                    key.to_str().unwrap(),
                    value.to_str().unwrap()
                ),
            },
            Commands::Raw(cmd) => cmd.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct NuModeOutput {
    pub env: HashMap<String, String>,
    pub dir: Option<PathBuf>,
}

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

use std::{collections::HashMap, fs, io, path::PathBuf};

use serde::Serialize;

use crate::fireplace::Fireplace;

/// Output the shell script that is to be executed upon selecting the given fireplace.
/// The script is written to the given output_path.
pub fn output_shell_cmd(
    fireplace: &Fireplace,
    output_path: &PathBuf,
    shell: Box<dyn ShellBackend>,
) -> io::Result<()> {
    std::fs::write(output_path, shell.out_file(fireplace)?)
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

pub trait ShellBackend {
    fn init(&self) -> &'static str;
    fn out_file(&self, fireplace: &Fireplace) -> io::Result<String>;
}

pub struct BashBackend;
pub struct ZshBackend;
pub struct NuBackend;

impl ShellBackend for BashBackend {
    fn init(&self) -> &'static str {
        r#"floo() {
            local tmp
            tmp=$(mktemp)
            export FLOO_OUTPUT_FILE="$tmp"
            command floo-bin "$@"
            if [ -s "$tmp" ]; then . "$tmp"; fi
            rm -f "$tmp"
            unset FLOO_OUTPUT_FILE
        }"#
    }
    fn out_file(&self, fireplace: &Fireplace) -> io::Result<String> {
        let mut lines = vec![];
        let floo_script = fireplace.get_directory().join(".floo");

        if floo_script.exists() {
            lines.push(format!(
                "FLOO_DIR='{}'",
                fireplace.get_directory().display()
            ));
            lines.push(format!("FLOO_NAME='{}'", fireplace.name));
            lines.push(format!("source '{}'", floo_script.display()));
        } else {
            for candidate in [".env", ".envrc"] {
                let p = fireplace.get_directory().join(candidate);
                if p.is_file() {
                    lines.push(format!("source '{}'", p.display()));
                }
            }
            lines.push(format!("cd '{}'", fireplace.get_directory().display()));
        }
        Ok(lines.join("\n"))
    }
}

impl ShellBackend for ZshBackend {
    fn init(&self) -> &'static str {
        BashBackend.init()
    }
    fn out_file(&self, fireplace: &Fireplace) -> io::Result<String> {
        BashBackend.out_file(fireplace)
    }
}

impl ShellBackend for NuBackend {
    fn init(&self) -> &'static str {
        r#"def --env --wrapped floo [...args] {
            let tmp = (mktemp)
            $env.FLOO_OUTPUT_FILE = $tmp
            ^floo-bin ...$args
            let out = (if (($tmp | path exists) and ((ls $tmp).0.size > 0b)) { open $tmp } else { {} })
            load-env $out.env
            cd $out.cwd }
            rm -rf $tmp
            hide-env FLOO_OUTPUT_FILE
        }"#
    }
    fn out_file(&self, fireplace: &Fireplace) -> io::Result<String> {
        let mut env: HashMap<String, String> = HashMap::new();
        let file = fireplace.get_directory().join(".env");
        if file.is_file() {
            let contents = fs::read_to_string(&file)?;
            for item in dotenvy::from_read_iter(contents.as_bytes()) {
                let (key, val) = item.map_err(io::Error::other)?;
                env.insert(key, val);
            }
        };
        let out = NuModeOutput {
            env,
            dir: fireplace.get_directory(),
        };
        serde_json::to_string(&out).map_err(io::Error::other)
    }
}

#[derive(Serialize)]
pub struct NuModeOutput {
    pub env: HashMap<String, String>,
    pub dir: PathBuf,
}

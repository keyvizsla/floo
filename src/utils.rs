use std::{env, path::PathBuf};

use crate::project::Project;

/// Remove a project from the given list of projects.
/// If the project is not contained, nothing happens.
pub fn remove_project(projects: &mut Vec<Project>, project_to_delete: &Project) {
    projects.retain(|p| p.name != project_to_delete.name);
}

pub fn replace_project(projects: &mut Vec<Project>, old_project: &Project, new_project: Project) {
    for i in 0..projects.len() {
        if projects[i].name == old_project.name {
            projects[i] = new_project.clone();
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

pub fn appdata_dir() -> PathBuf {
    let path = env::var("XDG_DATA_HOME").ok().map(PathBuf::from);
    if path.is_some() {
        return path.unwrap();
    }
    let home_directory =
        PathBuf::from(env::var("HOME").expect("Cannot deduce apdata path without HOME directory"));
    let floo_directory = home_directory.join(".local/share/floo/");
    if !floo_directory.exists() {
        std::fs::create_dir_all(&floo_directory).expect("Failed to create .floo directory");
    }
    return floo_directory;
}

/// Return the path to the default/configured
/// directory containing template startup scripts.
pub fn get_template_dir() -> PathBuf {
    appdata_dir().join("templates")
}

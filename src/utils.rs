use crate::project::Project;

/// Remove a project from the given list of projects.
/// If the project is not contained, nothing happens.
pub fn remove_project(projects: &mut Vec<Project>, project_to_delete: &Project) {
    projects.retain(|p| p.name != project_to_delete.name);
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

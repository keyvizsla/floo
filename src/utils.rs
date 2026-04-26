use crate::project::Project;

/// Remove a project from the given list of projects.
/// If the project is not contained, nothing happens.
pub fn remove_project(projects: &mut Vec<Project>, project_to_delete: &Project) {
    projects.retain(|p| p.name != project_to_delete.name);
}

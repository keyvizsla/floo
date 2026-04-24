use crate::project::Project;

pub enum Action {
    Quit,

    // Trigger the creation of a new (empty) fireplace
    CreateNewFireplace,

    // Add an existing fireplace with all properties in place to the app
    AddFireplace,

    Pick(Project),
    Noop,
}

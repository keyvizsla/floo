use crate::project::Project;

pub enum Action {
    Quit,
    CreateNewFireplace,
    Pick(Project),
    Noop,
}

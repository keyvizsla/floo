use crate::project::Project;

pub enum Action {
    Quit,
    Pick(Project),
}

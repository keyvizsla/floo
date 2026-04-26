use crate::project::Project;

#[derive(Clone)]
pub enum Action {
    Quit,
    AddFireplace(Project),
    DeleteFireplace(Project),
    Pick(Project),
    ClosePopup,
    Noop,
}

use crate::project::Project;

pub enum Action {
    Quit,
    AddFireplace(Project),
    DeleteFireplace(Project),
    Pick(Project),
    ClosePopup,
    Noop,
}

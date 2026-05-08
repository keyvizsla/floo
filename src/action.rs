use crate::{errors::FlooError, project::Project};

#[derive(Clone)]
pub enum Action {
    Quit,
    AddFireplace(Project),
    DeleteFireplace(Project),
    Pick(Project),
    OpenCreationPopup(Option<Project>),
    ClosePopup,
    EditNotes(Project),
    ReplaceProject { old: Project, new: Project },
    Error(FlooError),
    Noop,
}

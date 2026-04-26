use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect, Spacing};
use ratatui::symbols::merge::MergeStrategy;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use crate::action::Action;
use crate::components::component::{Component, ComponentCreationError};
use crate::components::deletion_popup::DeletionPopup;
use crate::components::new_fireplace_popup::NewFireplaceComponent;
use crate::project::Project;

pub struct MainScreen {
    projects: Vec<Project>,
    selected_project: usize,
    deletion_popup: Option<DeletionPopup>,
    creation_popup: Option<NewFireplaceComponent>,
}

impl MainScreen {
    pub fn init_with_projects(projects: Vec<Project>) -> Self {
        let selected_project = 0;
        MainScreen {
            projects,
            selected_project,
            deletion_popup: None,
            creation_popup: None,
        }
    }

    fn selected_project(&self) -> Project {
        self.projects[self.selected_project].clone()
    }

    fn select_next_project(&mut self) {
        self.selected_project += 1;
        if self.selected_project >= self.projects.len() {
            self.selected_project = 0;
        }
    }

    fn select_previous_project(&mut self) {
        if self.selected_project == 0 {
            self.selected_project = self.projects.len() - 1;
        } else {
            self.selected_project -= 1;
        }
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
        self.selected_project = self.projects.len() - 1;
    }

    pub fn remove_project(&mut self, project: &Project) {
        self.projects.retain(|p| p.name != project.name);
        if self.projects.len() == 0 {
            self.selected_project = 0;
        } else if self.selected_project >= self.projects.len() {
            self.selected_project = self.projects.len() - 1;
        }
    }
}

impl Component for MainScreen {
    fn init(&mut self) -> Result<(), ComponentCreationError> {
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Action {
        if event.is_none() {
            return Action::Noop;
        }

        if let Some(popup) = &mut self.deletion_popup {
            match popup.handle_events(event) {
                Action::ClosePopup => self.deletion_popup = None,
                Action::DeleteFireplace(project) => {
                    self.deletion_popup = None;
                    return Action::DeleteFireplace(project);
                }
                _ => {
                    return Action::Noop;
                }
            }
            return Action::Noop;
        }

        if let Some(popup) = &mut self.creation_popup {
            match popup.handle_events(event) {
                Action::ClosePopup => self.creation_popup = None,
                Action::AddFireplace(project) => {
                    self.creation_popup = None;
                    return Action::AddFireplace(project);
                }
                _ => {
                    return Action::Noop;
                }
            }
            return Action::Noop;
        }

        match event.unwrap() {
            Event::Key(e) => self.handle_key_events(e),
            _ => Action::Noop,
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.select_next_project();
                Action::Noop
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.select_previous_project();
                Action::Noop
            }
            KeyCode::Char('n') | KeyCode::Char('%') => {
                self.creation_popup = Some(NewFireplaceComponent::new());
                let _ = self.creation_popup.as_mut().unwrap().init();
                return Action::Noop;
            }
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Char('d') => {
                let mut popup = DeletionPopup::new(self.selected_project());
                let _ = popup.init();
                self.deletion_popup = Some(popup);
                Action::Noop
            }
            KeyCode::Enter => Action::Pick(self.selected_project()),
            _ => Action::Noop,
        }
    }

    fn handle_mouse_events(&mut self, _mouse: MouseEvent) -> Action {
        Action::Noop
    }

    fn update(&mut self, _action: Action) -> Action {
        Action::Noop
    }

    fn render(&mut self, f: &mut Frame, rect: Rect) {
        let list_items: Vec<ListItem> = self
            .projects
            .iter()
            .map(|i| ListItem::new(i.name.clone()))
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .title(" Select an Item (j/k or Arrows) ")
                    .borders(Borders::ALL)
                    .merge_borders(MergeStrategy::Exact),
            )
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true);
        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2])
            .spacing(Spacing::Overlap(1))
            .areas(f.area());

        let mut list_state = ListState::default().with_selected(Some(self.selected_project));

        f.render_stateful_widget(list, left, &mut list_state);

        let raw_project_description = self
            .selected_project()
            .get_description()
            .unwrap_or_else(|| "No description available.".to_string());

        let parsed_text = tui_markdown::from_str(raw_project_description.as_str());

        let paragraph = Paragraph::new(parsed_text)
            .block(
                Block::bordered()
                    .title("Project Description")
                    .merge_borders(MergeStrategy::Exact),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(paragraph, right);

        if let Some(deletion_popup) = &mut self.deletion_popup {
            deletion_popup.render(f, rect);
        }

        if let Some(creation_popup) = &mut self.creation_popup {
            creation_popup.render(f, rect);
        }
    }
}


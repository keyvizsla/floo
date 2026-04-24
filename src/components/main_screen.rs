use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect, Spacing};
use ratatui::symbols::merge::MergeStrategy;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use rusqlite::fallible_iterator::FallibleIterator;
use crate::action::Action;
use crate::components::component::{Component, ComponentCreationError};
use crate::project::Project;

pub struct MainScreen {
    projects: Vec<Project>,
    selected_project: usize,
}

impl MainScreen {
    pub fn init_with_projects(projects: Vec<Project>) -> Self {
        let selected_project = 0;
        MainScreen { projects, selected_project }
    }

    fn selected_project(&self) -> Project {
        self.projects[self.selected_project].clone()
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
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

        match event.unwrap() {
            Event::Key(e) => self.handle_key_events(e),
            _ => Action::Noop,
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match key.code {
            // KeyCode::Char('n') | KeyCode::Char('%') => {
            //     self.creation_popup = Some(NewFireplaceComponent::new());
            //     let _ = self.creation_popup.as_mut().unwrap().init();
            //     return Action::Noop;
            // },
            KeyCode::Char('q') => Action::Quit,
            _ => Action::Noop,
        }
    }

    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action {
        Action::Noop
    }

    fn update(&mut self, action: Action) -> Action {
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

        f.render_widget(
            paragraph,
            right,
        );
    }
}
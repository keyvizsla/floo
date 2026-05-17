/*
 * Copyright (C) 2026 Leon Degel-Koehn
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect, Spacing};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph, Wrap};
use ratatui_interact::components::{Tab, TabView, TabViewState};
use ratatui_interact::prelude::{ListPicker, ListPickerState, Toast, ToastState, ToastStyle};
use ratatui_interact::traits::Focusable;
use ratatui_interact::utils::render_markdown_to_lines;

use crate::action::Action;
use crate::components::component::{Component, ComponentCreationError};
use crate::components::deletion_popup::DeletionPopup;
use crate::components::help_popup::HelpPopup;
use crate::components::new_fireplace_popup::NewFireplaceComponent;
use crate::components::select_template::SelectTemplatePopup;
use crate::components::status_footer::StatusFooter;
use crate::errors::FlooError;
use crate::fireplace::Fireplace;
use crate::utils::{get_template_dir, remove_project, replace_project};

use ratatui::widgets::Widget;

#[derive(Default)]
pub struct MainScreen {
    projects: Vec<Fireplace>,
    description_scroll: u16,
    tab_state: TabViewState,
    list_state: ListPickerState,
    toast_state: ToastState,
    deletion_popup: Option<DeletionPopup>,
    creation_popup: Option<NewFireplaceComponent>,
    help_popup: Option<HelpPopup>,
    template_popup: Option<SelectTemplatePopup>,
}

impl MainScreen {
    pub fn init_with_projects(projects: Vec<Fireplace>) -> Self {
        let mut tab_state = TabViewState::new(2);
        let mut list_state = ListPickerState::new(projects.len());
        let toast_state = ToastState::new();
        tab_state.select(0);
        list_state.select_first();
        MainScreen {
            projects,
            description_scroll: 0,
            deletion_popup: None,
            creation_popup: None,
            help_popup: None,
            template_popup: None,
            tab_state,
            list_state,
            toast_state,
        }
    }

    fn selected_project(&self) -> Fireplace {
        self.projects[self.list_state.selected_index].clone()
    }

    fn select_next_project(&mut self) {
        if self.list_state.selected_index == self.list_state.total_items - 1 {
            self.list_state.select_first();
        } else {
            self.list_state.select_next();
        }
    }

    fn select_previous_project(&mut self) {
        if self.list_state.selected_index == 0 {
            self.list_state.select_last();
        } else {
            self.list_state.select_prev();
        }
    }

    pub fn add_project(&mut self, project: Fireplace) {
        self.projects.push(project);
        self.list_state = ListPickerState::new(self.projects.len());

        // Select the newly created project, users will likely want that one
        self.list_state.select_last();
    }

    pub fn remove_project(&mut self, project: &Fireplace) {
        remove_project(&mut self.projects, project);
        let old_idx = self.list_state.selected_index;
        self.list_state = ListPickerState::new(self.projects.len());
        self.list_state.select(old_idx);
    }

    fn render_tab_content(&self, idx: usize, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        match idx {
            0 => self.render_description_tab(area, buf),
            1 => self.render_notes_tab(area, buf),
            _ => {}
        }
    }

    fn render_description_tab(&self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let raw_project_description = self
            .selected_project()
            .get_description()
            .unwrap_or_else(|| "No description available.".to_string());

        let parsed_text = render_markdown_to_lines(raw_project_description.as_str());

        let paragraph = Paragraph::new(parsed_text)
            .scroll((self.description_scroll, 0))
            .wrap(Wrap { trim: false });
        paragraph.render(area, buf);

        if !self.selected_project().has_startup_script() {
            let message = "No startup script configured. Press `e` to create one from a template."
                .to_string();
            let footer = StatusFooter { message };
            Widget::render(footer, area, buf);
        }
    }

    fn render_notes_tab(&self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let notes = self.selected_project().notes;
        let parsed_text = render_markdown_to_lines(&notes);

        let paragraph = Paragraph::new(parsed_text).wrap(Wrap { trim: false });
        paragraph.render(area, buf);
    }

    fn render_notifications(&mut self, f: &mut Frame, area: Rect) {
        self.toast_state.clear_if_expired();
        if self.toast_state.get_message().is_none() {
            return;
        }
        let message = self.toast_state.get_message().unwrap();
        let toast = Toast::new(message).style(ToastStyle::Error);
        let target_dimensions = toast.calculate_area(area);
        let [_, toast_area_horizontal, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(target_dimensions.width),
            Constraint::Length(2),
        ])
        .areas(area);
        let [_, toast_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(target_dimensions.height),
            Constraint::Length(1),
        ])
        .areas(toast_area_horizontal);

        // We don't use render_with_clear on purpose, since that messes with the alignment of the toast
        Clear.render(toast_area, f.buffer_mut());
        toast.render(toast_area, f.buffer_mut());
    }
}

impl Component for MainScreen {
    fn init(&mut self) -> Result<(), ComponentCreationError> {
        self.projects.sort_by_key(|x| x.last_accessed);
        self.projects.reverse();
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Action {
        self.toast_state.clear_if_expired();
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

        if let Some(popup) = &mut self.help_popup {
            match popup.handle_events(event) {
                Action::ClosePopup => self.help_popup = None,
                _ => {
                    return Action::Noop;
                }
            }
            return Action::Noop;
        }

        if let Some(popup) = &mut self.template_popup {
            match popup.handle_events(event) {
                Action::ClosePopup => self.template_popup = None,
                Action::SelectTemplate {
                    template,
                    project: _,
                } => {
                    self.template_popup = None;
                    return Action::SelectTemplate {
                        template,
                        project: Some(self.selected_project()),
                    };
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
        // TODO: Distinguish between notes and description scroll
        // based on the open tab
        if key.modifiers == KeyModifiers::CONTROL {
            if let KeyCode::Char('d') = key.code {
                self.description_scroll += 5;
                return Action::Noop;
            }
            if let KeyCode::Char('u') = key.code {
                if self.description_scroll >= 5 {
                    self.description_scroll -= 5;
                }
                return Action::Noop;
            }
        }
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
            KeyCode::Char('h') => {
                let mut popup = HelpPopup::new();
                let _ = popup.init();
                self.help_popup = Some(popup);
                Action::Noop
            }
            KeyCode::Char('e') => {
                if self.tab_state.selected_index == 1 {
                    Action::EditNotes(self.selected_project())
                } else if !self.selected_project().has_startup_script() {
                    let mut popup = SelectTemplatePopup::new(get_template_dir());
                    let _ = popup.init();
                    self.template_popup = Some(popup);
                    Action::Noop
                } else {
                    Action::Noop
                }
            }
            KeyCode::Tab => {
                if self.tab_state.selected_index == self.tab_state.total_tabs - 1 {
                    self.tab_state.select_first();
                } else {
                    self.tab_state.select_next();
                }
                Action::Noop
            }
            KeyCode::Enter => Action::Pick(self.selected_project()),
            _ => Action::Noop,
        }
    }

    fn handle_mouse_events(&mut self, _mouse: MouseEvent) -> Action {
        Action::Noop
    }

    fn update(&mut self, action: Action) -> Action {
        match action {
            Action::AddFireplace(project) => self.add_project(project),
            Action::DeleteFireplace(project) => self.remove_project(&project),
            Action::OpenCreationPopup(project) => {
                self.creation_popup = match project {
                    Some(p) => Some(NewFireplaceComponent::with_prefill(p)),
                    None => Some(NewFireplaceComponent::new()),
                };
                let _ = self.creation_popup.as_mut().unwrap().init();
            }
            Action::ReplaceFireplace {
                old: old_project,
                new: new_project,
            } => replace_project(&mut self.projects, &old_project, new_project),
            Action::Error(FlooError::DbUpdateError(msg)) => {
                self.toast_state.show(msg, 3000);
            }
            _ => {}
        }
        Action::Noop
    }

    fn render(&mut self, f: &mut Frame, rect: Rect) {
        let list_items: Vec<Line> = self
            .projects
            .iter()
            .map(|i| Line::from(i.name.clone()))
            .collect();

        let fireplace_picker = ListPicker::new(&list_items, &self.list_state);

        let title = Span::styled(
            " Select a Fireplace to travel to (Press `h` for help)",
            // TODO: See if there is a cleaner way to access the current style of ratatui interact
            self.tab_state.focused_style().add_modifier(Modifier::BOLD),
        );

        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2])
            .spacing(Spacing::Overlap(1))
            .areas(f.area());

        // We have the title as a seperate widget to enable spacial alignnment with
        // the righthand tabview.
        let [top_left, left] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(left);

        f.render_widget(title, top_left);
        f.render_widget(fireplace_picker, left);

        let tabs = vec![
            Tab::new("About").icon("\u{2139}"), // Info icon
            Tab::new("Notes").icon("📒"),       // Notes icon
        ];
        let tab_view = TabView::new(&tabs, &self.tab_state).content(|idx, area, buf| {
            self.render_tab_content(idx, area, buf);
        });

        f.render_widget(tab_view, right);

        if let Some(deletion_popup) = &mut self.deletion_popup {
            deletion_popup.render(f, rect);
        }

        if let Some(creation_popup) = &mut self.creation_popup {
            creation_popup.render(f, rect);
        }

        if let Some(help_popup) = &mut self.help_popup {
            help_popup.render(f, rect);
        }

        if let Some(popup) = &mut self.template_popup {
            popup.render(f, rect);
        }

        self.render_notifications(f, rect);
    }
}

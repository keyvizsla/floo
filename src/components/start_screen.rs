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

use crate::action::Action;
use crate::components::component::{Component, ComponentCreationError};
use crate::components::new_fireplace_popup::NewFireplaceComponent;
use crate::errors::FlooError;
use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style, Stylize, Text};
use ratatui::widgets::Widget;
use ratatui::widgets::{Clear, Paragraph};
use ratatui_interact::components::{Toast, ToastState, ToastStyle};

#[derive(Default)]
pub struct StartScreen {
    creation_popup: Option<NewFireplaceComponent>,
    toast_state: ToastState,
}

impl StartScreen {
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

    fn handle_open_popup_event(&mut self, event: Event) -> Option<Action> {
        if let Some(popup) = &mut self.creation_popup {
            match popup.handle_events(Some(event)) {
                Action::ClosePopup => self.creation_popup = None,
                Action::AddFireplace(project) => {
                    self.creation_popup = None;
                    return Some(Action::AddFireplace(project));
                }
                _ => {
                    return Some(Action::Noop);
                }
            }
            return Some(Action::Noop);
        }

        None
    }
}

impl Component for StartScreen {
    fn init(&mut self) -> Result<(), ComponentCreationError> {
        self.creation_popup = None;
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        self.toast_state.clear_if_expired();
        if let Some(action) = self.handle_open_popup_event(Event::Key(key)) {
            return action;
        }

        match key.code {
            KeyCode::Char('n') | KeyCode::Char('%') => {
                self.creation_popup = Some(NewFireplaceComponent::new());
                let _ = self.creation_popup.as_mut().unwrap().init();
                Action::Noop
            }
            KeyCode::Char('q') => Action::Quit,
            _ => Action::Noop,
        }
    }

    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action {
        self.toast_state.clear_if_expired();
        self.handle_open_popup_event(Event::Mouse(mouse))
            .unwrap_or(Action::Noop)
    }

    fn update(&mut self, action: Action) -> Action {
        match action {
            Action::OpenCreationPopup(project) => {
                self.creation_popup = match project {
                    Some(p) => Some(NewFireplaceComponent::with_prefill(p)),
                    None => Some(NewFireplaceComponent::new()),
                };
                let _ = self.creation_popup.as_mut().unwrap().init();
            }
            Action::Error(FlooError::DbUpdateError(msg)) => {
                self.toast_state.show(msg, 3000);
            }
            _ => {}
        }
        Action::Noop
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Length(12),
                Constraint::Percentage(30),
            ])
            .split(area);

        let ascii_logo = "
   ███████╗██╗      ██████╗  ██████╗
    ██╔════╝██║     ██╔═══██╗██╔═══██╗
    █████╗  ██║     ██║   ██║██║   ██║
    ██╔══╝  ██║     ██║   ██║██║   ██║
    ██║     ███████╗╚██████╔╝╚██████╔╝
    ╚═╝     ╚══════╝ ╚═════╝  ╚═════╝ ";
        let version_str = format!(" v{}", env!("CARGO_PKG_VERSION"));
        let version_hint = Span::styled(
            version_str,
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        );

        let status_line =
            Line::from("Your floo network is dark, no fireplaces are connected.").bold();
        let hint_line = Line::from(vec![
            Span::raw("To add a destination, press "),
            Span::styled("n", Style::default().bold().fg(Color::LightCyan)),
            Span::raw(" or "),
            Span::styled("%", Style::default().bold().fg(Color::LightCyan)),
        ])
        .fg(Color::DarkGray);

        let mut final_content = Text::from(ascii_logo);
        final_content.lines.push(Line::from(version_hint));
        final_content.lines.push(Line::from("")); // Spacer
        final_content.lines.push(status_line);
        final_content.lines.push(hint_line);

        let paragraph = Paragraph::new(final_content).alignment(Alignment::Center);

        f.render_widget(paragraph, chunks[1]);

        if let Some(popup) = &mut self.creation_popup {
            popup.render(f, area);
        }

        self.render_notifications(f, area);
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "test-docker-linux")]
    mod docker_linux_tests {
        use crossterm::event::KeyModifiers;

        use super::super::*;
        use crate::{components::start_screen::StartScreen, test_utils::TuiFixture};

        /// Verify the layout of the base start screen.
        #[test]
        fn layout_basic() {
            let mut fx = TuiFixture::default();
            let mut screen = StartScreen::default();

            fx.render_and_snapshot(|frame| {
                screen.render(frame, frame.area());
            });
        }

        /// Verify that the fireplace creation popup is opened after the
        /// corresponding keys are pressed.
        #[test]
        fn layout_open_popup() {
            let mut fx = TuiFixture::default();
            let mut screen = StartScreen::default();
            let input_event = Event::Key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));

            screen.handle_events(Some(input_event));

            fx.render_and_snapshot(|frame| {
                screen.render(frame, frame.area());
            });
        }
    }
}

use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style, Stylize, Text};
use ratatui::widgets::Paragraph;
use crate::action::Action;
use crate::components::component::{Component, ComponentCreationError};
use crate::components::new_fireplace_popup::NewFireplaceComponent;

#[derive(Default)]
pub struct StartScreen {
    creation_popup: Option<NewFireplaceComponent>
}

impl Component for StartScreen {
    fn init(&mut self) -> Result<(), ComponentCreationError> {
        self.creation_popup = None;
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Action {
        if event.is_none() {
            return Action::Noop;
        }

        if let Some(popup) = &mut self.creation_popup {
            match popup.handle_events(event) {
                Action::ClosePopup => { self.creation_popup = None },
                Action::AddFireplace(project) => {
                    self.creation_popup = None;
                    return Action::AddFireplace(project);
                }
                _ => {
                    return Action::Noop;
                },
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
            KeyCode::Char('n') | KeyCode::Char('%') => {
                self.creation_popup = Some(NewFireplaceComponent::new());
                let _ = self.creation_popup.as_mut().unwrap().init();
                return Action::Noop;
            },
            KeyCode::Char('q') => Action::Quit,
            _ => Action::Noop,
        }
    }

    fn handle_mouse_events(&mut self, _mouse: MouseEvent) -> Action {
        Action::Noop
    }

    fn update(&mut self, _action: Action) -> Action {
        Action::Noop
    }

    fn render(&mut self, f: &mut Frame, _: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Length(12),
                Constraint::Percentage(30),
            ])
            .split(f.area());

        let ascii_logo = "
   ███████╗██╗      ██████╗  ██████╗
    ██╔════╝██║     ██╔═══██╗██╔═══██╗
    █████╗  ██║     ██║   ██║██║   ██║
    ██╔══╝  ██║     ██║   ██║██║   ██║
    ██║     ███████╗╚██████╔╝╚██████╔╝
    ╚═╝     ╚══════╝ ╚═════╝  ╚═════╝ ";
        let version_hint = Span::styled(
            " v0.1.0",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        );

        let status_line = Line::from("Your floo network is dark, no fireplaces are connected.").bold();
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
            // TODO: dont render popup on full area
            popup.render(f, f.area());
        }

    }
}
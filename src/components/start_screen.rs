use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use crossterm::event::Event::Key;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style, Stylize, Text};
use ratatui::widgets::Paragraph;
use crate::action::Action;
use crate::components::component::{Component, ComponentCreationError};

#[derive(Default)]
pub struct StartScreen {
    pub visible: bool,
}
impl Component for StartScreen {
    fn init(&mut self) -> Result<(), ComponentCreationError> {
        self.visible = true;
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
            KeyCode::Char('n') | KeyCode::Char('%') => Action::CreateNewFireplace,
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

    fn render(&mut self, f: &mut Frame, _: Rect) {
        if !self.visible {
            return;
        }
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
    }
}
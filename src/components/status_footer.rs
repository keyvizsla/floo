use crossterm::event::{Event, KeyEvent, MouseEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};
use ratatui_interact::theme::Theme;

use crate::{
    action::Action,
    components::component::{Component, ComponentCreationError},
};

#[derive(Clone)]
pub struct StatusFooter {
    pub message: String,
}

impl Widget for StatusFooter {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let theme = Theme::dark();
        let num_lines = self.message.lines().count();
        let text = Paragraph::new(self.message)
            .block(
                Block::new()
                    .borders(Borders::TOP)
                    .style(theme.palette.border_accent),
            )
            .wrap(Wrap { trim: true })
            .fg(theme.palette.error)
            .bold()
            .bg(theme.palette.bg);
        let [_, bottom_area] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1 + num_lines as u16),
        ])
        .areas(area);
        text.render(bottom_area, buf);
    }
}

impl Component for StatusFooter {
    fn init(&mut self) -> Result<(), ComponentCreationError> {
        Ok(())
    }

    fn handle_events(&mut self, _: Option<Event>) -> Action {
        Action::Noop
    }

    fn handle_key_events(&mut self, _: KeyEvent) -> Action {
        Action::Noop
    }

    fn handle_mouse_events(&mut self, _: MouseEvent) -> Action {
        Action::Noop
    }

    fn update(&mut self, _action: Action) -> Action {
        Action::Noop
    }

    // The footer stretches the full width of the rect
    // but only occupies the bottom of it.
    fn render(&mut self, f: &mut Frame, rect: Rect) {
        <Self as Widget>::render(self.clone(), rect, f.buffer_mut());
    }
}

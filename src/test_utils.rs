use ratatui::{backend::TestBackend, prelude::*};

pub struct TuiFixture {
    terminal: Terminal<TestBackend>,
}

impl TuiFixture {
    /// Create a new fixture with a specific terminal size
    pub fn new(w: u16, h: u16) -> Self {
        let backend = TestBackend::new(w, h);
        let terminal = Terminal::new(backend).unwrap();

        unsafe {
            std::env::set_var("TERM", "xterm-256color");
            std::env::set_var("COLORTERM", "truecolor");
            std::env::set_var("TZ", "UTC");
        }

        Self { terminal }
    }

    /// Renders a component using a closure that provides the Frame,
    /// then automatically captures and asserts the snapshot.
    pub fn render_and_snapshot<F>(&mut self, render_fn: F)
    where
        F: FnOnce(&mut Frame),
    {
        self.terminal.draw(render_fn).unwrap();
        let buffer = self.terminal.backend().buffer();
        insta::assert_snapshot!(format!("{buffer:?}"));
    }
}

impl Default for TuiFixture {
    /// Creates a fixture with the default test terminal dimensions
    fn default() -> Self {
        let default_width = 80;
        let default_height = 20;
        Self::new(default_width, default_height)
    }
}

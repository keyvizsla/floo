use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::io;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let items = vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"];
    let mut state = ListState::default();
    state.select(Some(0));

    let mut selected_item = None;

    // 3. Main Loop
    loop {
        terminal.draw(|f| {
            let list_items: Vec<ListItem> = items.iter().map(|i| ListItem::new(*i)).collect();

            let list = List::new(list_items)
                .block(
                    Block::default()
                        .title(" Select an Item (j/k or Arrows) ")
                        .borders(Borders::ALL),
                )
                .highlight_symbol(">> ")
                .repeat_highlight_symbol(true);

            f.render_stateful_widget(list, f.area(), &mut state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('j') | KeyCode::Down => {
                    let i = match state.selected() {
                        Some(i) if i >= items.len() - 1 => 0,
                        Some(i) => i + 1,
                        None => 0,
                    };
                    state.select(Some(i));
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    let i = match state.selected() {
                        Some(i) if i == 0 => items.len() - 1,
                        Some(i) => i - 1,
                        None => 0,
                    };
                    state.select(Some(i));
                }
                KeyCode::Enter => {
                    if let Some(i) = state.selected() {
                        selected_item = Some(items[i].to_string());
                    }
                    break;
                }
                _ => {}
            }
        }
    }

    // 4. Cleanup Terminal (Crucial for a usable terminal after exit)
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // 5. Output to stdout
    if let Some(item) = selected_item {
        println!("{}", item);
    }

    Ok(())
}

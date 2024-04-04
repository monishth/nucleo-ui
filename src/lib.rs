/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

pub mod directory;

use app::{App, AppResult};
use event::{Event, EventHandler};
use handler::handle_key_events;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout, Write};
use tui::Tui;

pub fn launch_ui(lists: Option<Vec<String>>) -> AppResult<()> {
    // Create an application.
    let mut app = match lists {
        None => App::new(directory::get_directories(), true),
        Some(lists) => App::new(lists, false),
    };

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(50);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }

        if app.result.is_some() {
            app.quit();
        }
    }

    // Exit the user interface.
    tui.exit()?;
    if let Some(result) = app.result {
        if let Err(err) = writeln!(stdout(), "{}", result) {
            return Err(err.into());
        };
    }

    Ok(())
}

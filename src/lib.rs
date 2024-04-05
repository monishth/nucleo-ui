#![feature(let_chains)]
use std::io::Write;
/// Application.
pub mod model;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

pub mod directory;

pub mod cli;

use event::{Event, EventHandler};
use handler::handle_key_events;
use model::FuzzyMatchModel;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout};
use tui::Tui;

use crate::cli::Cli;

pub fn launch_ui(lists: Option<Vec<String>>, args: Option<Cli>) -> color_eyre::Result<()> {
    let args = args.unwrap_or_default();
    let path = args.path.unwrap_or(std::env::current_dir()?);

    let mut model = match lists {
        None => FuzzyMatchModel::new(
            directory::get_directories(path, args.min_depth, args.max_depth, args.directory)?,
            args.directory,
        ),
        Some(lists) => FuzzyMatchModel::new(lists, args.directory),
    };

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(50);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while model.running {
        // Render the user interface.
        tui.draw(&mut model)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => model.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut model)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }

        if model.result.is_some() {
            model.quit();
        }
    }

    // Exit the user interface.
    tui.exit()?;
    if let Some(result) = model.result {
        if let Err(err) = writeln!(stdout(), "{}", result) {
            return Err(err.into());
        };
    }

    Ok(())
}

use crate::model::FuzzyMatchModel;
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(crate) fn handle_key_events(key_event: KeyEvent, model: &mut FuzzyMatchModel) -> Result<()> {
    match key_event.code {
        KeyCode::Enter => {
            model.select();
        }
        KeyCode::Up => {
            model.up();
        }
        KeyCode::Down => {
            model.down();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                model.quit();
            } else {
                model.enter_char('c');
            }
        }
        KeyCode::Esc => {
            model.quit();
        }
        KeyCode::Char(c) => {
            model.enter_char(c);
        }
        KeyCode::Backspace => {
            model.delete_char();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

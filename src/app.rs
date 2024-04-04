use std::{error, sync::Arc};

use nucleo::Nucleo;
use ratatui::widgets::ListState;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    pub running: bool,
    pub result: Option<String>,
    pub input: String,
    pub prev_input: String,
    pub items: StatefulList,
    cursor_position: usize,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(list: impl IntoIterator<Item = String>, optimise_directories: bool) -> Self {
        let mut config = nucleo::Config::DEFAULT;
        if optimise_directories {
            config = config.match_paths();
        }
        let nucleo = Nucleo::new(config, Arc::new(|| {}), None, 1);
        let injector = nucleo.injector();

        for str in list {
            injector.push(str, |s, dst| dst[0] = s.to_owned().into());
        }

        Self {
            running: true,
            input: String::new(),
            result: None,
            prev_input: String::new(),
            cursor_position: 0,
            items: StatefulList::new(nucleo),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.items.tick();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn select(&mut self) {
        if let Some(selected) = self.items.state.selected() {
            self.result = self
                .items
                .items
                .snapshot()
                .get_item(selected as u32)
                .map(|s| s.data.to_owned());
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.prev_input.clone_from(&self.input);
        self.input.insert(self.cursor_position, new_char);

        self.move_cursor_right();
        self.items.update_input(&self.input, &self.prev_input);
    }

    pub fn up(&mut self) {
        self.items.next();
    }

    pub fn down(&mut self) {
        self.items.previous();
    }

    pub fn delete_char(&mut self) {
        self.prev_input.clone_from(&self.input);
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
        self.items.update_input(&self.input, &self.prev_input);
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }
}

pub struct StatefulList {
    pub state: ListState,
    pub items: Nucleo<String>,
    pub last_selected: Option<usize>,
}

impl StatefulList {
    fn new(matcher: Nucleo<String>) -> StatefulList {
        StatefulList {
            state: ListState::default().with_selected(Some(0)),
            items: matcher,
            last_selected: None,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= (self.items.snapshot().matched_item_count() as usize).saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    (self.items.snapshot().matched_item_count() as usize).saturating_sub(1)
                } else {
                    i.saturating_sub(1)
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn tick(&mut self) {
        self.items.tick(50);
    }

    fn update_input(&mut self, input: &str, original_input: &str) {
        self.state.select(Some(0));
        self.items.pattern.reparse(
            0,
            input,
            nucleo::pattern::CaseMatching::Smart,
            nucleo::pattern::Normalization::Smart,
            input.starts_with(original_input),
        );
    }

    // fn unselect(&mut self) {
    //     let offset = self.state.offset();
    //     self.last_selected = self.state.selected();
    //     self.state.select(None);
    //     *self.state.offset_mut() = offset;
    // }
}

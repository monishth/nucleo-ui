use std::sync::Arc;

use nucleo::{Matcher, Nucleo, Utf32String};
use ratatui::widgets::ListState;

/// Application result type.

/// Application.
pub struct FuzzyMatchModel {
    pub running: bool,
    pub result: Option<String>,
    pub input: String,
    prev_input: String,
    pub items: StatefulList,
    highlight_matcher: Matcher,
    cursor_position: usize,
    indices: Vec<u32>,
    pub snapshot: Snapshot,
    pub height: u32,
}

pub struct Snapshot {
    pub matched_items: Vec<(String, Vec<u32>)>,
    pub matched_item_count: usize,
    pub total_item_count: usize,
}

impl FuzzyMatchModel {
    pub fn new(list: impl IntoIterator<Item = String>, optimise_directories: bool) -> Self {
        let mut config = nucleo::Config::DEFAULT;
        if optimise_directories {
            config = config.match_paths();
        }
        let nucleo: Nucleo<Utf32String> = Nucleo::new(config, Arc::new(|| {}), None, 1);
        let injector = nucleo.injector();

        for str in list {
            injector.push(Utf32String::from(str), |s, dst| s.clone_into(&mut dst[0]));
        }

        Self {
            running: true,
            input: String::new(),
            result: None,
            height: 0,
            prev_input: String::new(),
            cursor_position: 0,
            highlight_matcher: Matcher::new(nucleo::Config::DEFAULT.match_paths()),
            items: StatefulList::new(nucleo),
            indices: Vec::new(),
            snapshot: Snapshot {
                matched_items: vec![],
                matched_item_count: 0,
                total_item_count: 0,
            },
        }
    }

    fn update_snapshot(&mut self) {
        let snap = self.items.items.snapshot();
        let count = snap.matched_item_count();
        let n = std::cmp::min(count, self.height);
        let matched_items = snap.matched_items(..n);
        let mut vec = vec![];
        for item in matched_items {
            let _score = self.items.items.pattern.column_pattern(0).indices(
                item.data.slice(..),
                &mut self.highlight_matcher,
                &mut self.indices,
            );
            self.indices.sort_unstable();
            self.indices.dedup();

            let indices = self.indices.drain(..).collect();
            vec.push((item.data.to_owned().to_string(), indices));
        }
        self.snapshot.matched_items.clear();
        self.snapshot.matched_items.extend(vec);
        self.snapshot.matched_item_count = snap.matched_item_count() as usize;
        self.snapshot.total_item_count = snap.item_count() as usize;
    }

    pub fn tick(&mut self) {
        self.items.tick();
        self.update_snapshot();
    }

    pub fn update_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn snapshot(&mut self) -> &Snapshot {
        &self.snapshot
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn select(&mut self) {
        if let Some(selected) = self.items.state.selected() {
            self.result = self
                .snapshot
                .matched_items
                .get(selected)
                .map(|s| s.0.to_owned());
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
    pub items: Nucleo<Utf32String>,
    pub last_selected: Option<usize>,
}

impl StatefulList {
    fn new(matcher: Nucleo<Utf32String>) -> StatefulList {
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
        self.items.tick(100);
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
}

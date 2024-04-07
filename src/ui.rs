use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{LineGauge, List, ListDirection, ListItem, Paragraph},
    Frame,
};

use crate::model::FuzzyMatchModel;

/// Renders the user interface widgets.
pub(crate) fn render(model: &mut FuzzyMatchModel, frame: &mut Frame) {
    let area = frame.size();
    let vertical = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
    ]);
    let [top, line_area, bottom] = vertical.areas(area);
    frame.render_widget(
        Paragraph::new(format!(">> {}", &model.input))
            .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        bottom,
    );
    model.update_height(area.height as u32);
    let mut state = model.items.state.clone();
    let snapshot = model.snapshot();
    frame.render_widget(
        LineGauge::default()
            .gauge_style(Style::default().fg(Color::White))
            .line_set(symbols::line::THICK)
            .ratio(1.0)
            .label(format!(
                "{}/{}",
                snapshot.matched_item_count, snapshot.total_item_count
            )),
        line_area,
    );

    let list = List::new(snapshot.matched_items.iter().map(|item| {
        let lines = split_highlights(&item.0, &item.1);
        let line = Line::from(
            lines
                .into_iter()
                .map(|(text, highlighted)| {
                    Span::styled(
                        text,
                        if highlighted {
                            Style::default()
                                .fg(Color::Red)
                                .add_modifier(Modifier::UNDERLINED | Modifier::ITALIC)
                        } else {
                            Style::default().fg(Color::White)
                        },
                    )
                })
                .collect::<Vec<Span>>(),
        );
        ListItem::new(line)
    }))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().bg(Color::Rgb(63, 63, 63)))
    .highlight_symbol(">>")
    .repeat_highlight_symbol(true)
    .direction(ListDirection::BottomToTop);

    frame.render_stateful_widget(list, top, &mut state);
}

/// Turns a string and list of ordered highlighted indicies into a list of tuples
/// of substrings and whether they are highlighted.
fn split_highlights<'a>(input: &'a str, indices: &Vec<u32>) -> Vec<(&'a str, bool)> {
    if indices.is_empty() {
        return vec![(input, false)];
    }
    let string_length = input.chars().count();
    let mut result = Vec::new();

    let mut is_highlighted = vec![false; string_length];
    for index in indices {
        is_highlighted[*index as usize] = true;
    }

    let mut current_string = 0;
    let mut current_highlight = is_highlighted[0];

    // Wanted to use char_indicies but I think nucleo gives us indices into the array
    for (i, _) in input.chars().enumerate() {
        let should_highlight = is_highlighted[i];

        if should_highlight != current_highlight {
            if i > current_string {
                result.push((&input[current_string..i], current_highlight));
                current_string = i;
            }
            current_highlight = should_highlight;
        }
    }

    if current_string < input.len() {
        result.push((&input[current_string..], current_highlight));
    }

    result
}

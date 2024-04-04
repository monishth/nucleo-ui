use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{LineGauge, List, ListDirection, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let snapshot = app.items.items.snapshot();
    let total_items = snapshot.item_count();
    let matched_item_count = snapshot.matched_item_count();

    let area = frame.size();
    let vertical = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
    ]);
    let [top, line_area, bottom] = vertical.areas(area);
    frame.render_widget(
        LineGauge::default()
            .gauge_style(Style::default().fg(Color::White))
            .line_set(symbols::line::THICK)
            .ratio(1.0)
            .label(format!("{}/{}", matched_item_count, total_items)),
        line_area,
    );
    frame.render_widget(
        Paragraph::new(format!(">> {}", &app.input))
            .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        bottom,
    );

    let list = List::new(
        snapshot
            .matched_items(..)
            .map(|item| ListItem::new(item.data.as_str())),
    )
    .style(Style::default().fg(Color::White))
    .highlight_style(
        Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::ITALIC),
    )
    .highlight_symbol(">>")
    .repeat_highlight_symbol(true)
    .direction(ListDirection::BottomToTop);
    frame.render_stateful_widget(list, top, &mut app.items.state);
}

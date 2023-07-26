#![allow(unused)]

use std::rc::Rc;

use crate::{
    app::App,
    mode::{DashState, Mode},
    utils::{centered_area, new_block},
};
use tui::{
    prelude::*,
    symbols::DOT,
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Tabs},
};

const BANNER: &str = include_str!("../../banner.txt");

pub fn render<B: Backend>(app: &mut App, chunk: Rect, frame: &mut Frame<'_, B>) {
    if app.mode.is_dashboard(DashState::Default) {
        let chunks = chunks(chunk);

        let banner = banner();
        let list = list(chunks[1]);

        frame.render_widget(banner, chunks[0]);
        frame.render_widget(list, chunks[1]);
    }
}

fn chunks(chunk: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Percentage(46)])
        .split(chunk)
}

fn banner() -> Paragraph<'static> {
    Paragraph::new(BANNER).alignment(Alignment::Center).block(
        new_block("")
            .borders(Borders::NONE)
            .padding(Padding::vertical(5)),
    )
}

fn list<'a>(chunk: Rect) -> List<'a> {
    let a = " Item 11111111111111111111";
    let b = symbols::line::THICK_VERTICAL_RIGHT.to_string() + a;
    let b = Box::leak(Box::new(b));
    let b = b.as_str();
    let items = [ListItem::new(b), ListItem::new(b), ListItem::new(b)];
    List::new(items)
        .block(
            new_block("Dashboard")
                .title_alignment(Alignment::Center)
                .padding(Padding::new(chunk.width / 2 - a.len() as u16 / 2, 0, 0, 0)),
        )
        .start_corner(Corner::BottomRight)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
}

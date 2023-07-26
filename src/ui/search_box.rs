#![allow(unused)]

use std::rc::Rc;

use crate::{
    app::App,
    utils::{self, centered_area, centered_line, new_block},
};
use tui::{
    prelude::*,
    widgets::{Clear, Padding, Paragraph, Wrap},
};
use tui_textarea::TextArea;

pub fn render<B: Backend>(app: &mut App, chunk: Rect, frame: &mut Frame<'_, B>) {
    if app.mode.is_search() {
        let chunks = chunks(chunk);
        let search_pattern = &app.search_line.lines()[0];
        let mut search_err = None;

        let (title, color) = match app.textarea.set_search_pattern(search_pattern) {
            Err(err) => {
                let err = err.to_string();
                let err = utils::wrap_text(&err, chunks[1].width - 2);
                search_err = Some(
                    Paragraph::new(err).block(new_block("err").padding(Padding::horizontal(2))),
                );
                ("Search/Err", Color::LightRed)
            }
            Ok(_) => ("Search/Ok", Color::LightGreen),
        };

        let (block, style) = (new_block(title), Style::default().fg(color));

        app.search_line.set_block(block);
        app.search_line.set_style(style);

        frame.render_widget(Clear, chunk);
        frame.render_widget(app.search_line.widget(), chunks[0]);

        if let Some(search_err) = search_err {
            frame.render_widget(search_err.style(style), chunks[1]);
        }
    }
}

fn chunks(chunk: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(90)])
        .split(chunk)
}

// #![allow(unused)]

mod dashboard;
mod help_box;
mod search_box;
mod status_line;
mod tabs;
mod textarea;

use std::rc::Rc;

use crate::{app::App, utils::centered_area};
use tui::prelude::*;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let chunks = chunks(frame.size());

    let search_box_chunk = centered_area(30, 40, chunks[0]);

    // dashboard::render(app, chunks[0], frame);

    textarea::render(app, chunks[0], frame);
    status_line::render(app, chunks[1], frame);
    search_box::render(app, search_box_chunk, frame);
    help_box::render(app, frame.size(), frame);
}

fn chunks(chunk: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(96), Constraint::Percentage(4)])
        .split(chunk)
}

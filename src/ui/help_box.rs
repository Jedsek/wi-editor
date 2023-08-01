use crate::{
    app::App,
    utils::{centered_area, new_block},
};
use tui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

pub fn render<B: Backend>(app: &mut App, chunk: Rect, frame: &mut Frame<'_, B>) {
    if app.mode.is_help() {
        let center_chunk = centered_area(50, 50, chunk);
        let help_box = Paragraph::new("hhh").block(new_block("Help"));

        frame.render_widget(Clear, center_chunk);
        frame.render_widget(help_box, center_chunk);
    }
}

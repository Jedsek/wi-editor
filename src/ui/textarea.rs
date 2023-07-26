use crate::{app::App, utils::new_block};
use tui::prelude::*;

pub fn render<B: Backend>(app: &mut App, chunk: Rect, frame: &mut Frame<'_, B>) {
    let textarea = &mut app.textarea;

    let title = format!("{}", app.path.display());
    let block = new_block(title);
    textarea.set_block(block);

    let style = textarea.cursor_style();
    let style = if app.mode.is_input() {
        style.fg(Color::Black).bg(Color::Yellow)
    } else {
        style.fg(Color::Black).bg(Color::Cyan)
    };
    textarea.set_cursor_style(style);

    frame.render_widget(textarea.widget(), chunk);
}

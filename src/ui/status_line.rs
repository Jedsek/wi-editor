use crate::{app::App, mode::Mode};
use tui::{prelude::*, widgets::Paragraph};

pub fn render<B: Backend>(app: &mut App, chunk: Rect, frame: &mut Frame<'_, B>) {
    let echo_text = app.echo_text.as_str();
    let mode = app.mode;
    let echo_text_prefix = match mode {
        Mode::Command => ":",
        _ => "",
    };

    let mode_style = mode_style(&app.mode);

    let mode_line = Span::styled(format!("~> {} ", mode), mode_style);
    let echo_text = Span::from(String::new() + echo_text_prefix + echo_text);

    let status_line = vec![mode_line, echo_text]
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();
    let status_line = Paragraph::new(status_line);

    frame.render_widget(status_line, chunk);
}

fn mode_style(mode: &Mode) -> Style {
    let style = Style::default()
        .add_modifier(Modifier::BOLD | Modifier::ITALIC)
        .black();

    match mode {
        Mode::Normal => style.on_cyan(),
        Mode::Input => style.on_yellow(),
        Mode::Search => style.on_green(),
        Mode::Command => style.on_light_magenta(),
        _ => style,
    }
}

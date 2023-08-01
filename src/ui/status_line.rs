use crate::{app::App, mode::Mode};
use tui::{prelude::*, widgets::Paragraph};

pub fn render<B: Backend>(app: &mut App, chunk: Rect, frame: &mut Frame<'_, B>) {
    let echo_text = app.echo_text.as_str();
    let mode = app.mode;
    let echo_text_prefix = match mode {
        Mode::Command => ":",
        _ => "",
    };

    let mode_line_style = mode_line_style(app);
    let echo_text_style = echo_text_style(app);

    let mode_line = Span::styled(format!("~> {} ", mode), mode_line_style);
    let echo_text = Span::styled(
        String::new() + echo_text_prefix + echo_text,
        echo_text_style,
    );

    let status_line = vec![mode_line, echo_text]
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();
    let status_line = Paragraph::new(status_line);

    frame.render_widget(status_line, chunk);
}

fn mode_line_style(app: &App) -> Style {
    let style = Style::default()
        .add_modifier(Modifier::BOLD | Modifier::ITALIC)
        .black();

    match app.mode {
        Mode::Normal => style.on_light_cyan(),
        Mode::Input => style.on_light_yellow(),
        Mode::Command => style.on_light_magenta(),
        Mode::Search => style.on_light_blue(),
        Mode::Help => style.on_light_green(),
        _ => style,
    }
}

fn echo_text_style(app: &App) -> Style {
    let style = Style::default().add_modifier(Modifier::ITALIC | Modifier::BOLD);
    if app.mode.is_command() {
        style.add_modifier(Modifier::UNDERLINED)
    } else {
        style
    }
}

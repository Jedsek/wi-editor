use crossterm::{cursor::*, execute};
use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};
use textwrap::{wrap_algorithms::Penalties, WrapAlgorithm};
use tui::{
    prelude::*,
    widgets::{block::Title, Block, Borders},
};
use tui_textarea::TextArea;

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_line(percent_y: u16, chunk: Rect) -> Rect {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Length(3),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(chunk)[1]
}

pub fn centered_area(percent_x: u16, percent_y: u16, chunk: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(chunk);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn new_block<'a, T>(title: T) -> Block<'a>
where
    T: Into<Title<'a>>,
{
    Block::default()
        .title(title)
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
}

pub fn show_cursor(flag: bool) {
    let mut stderr = io::stderr();
    if flag {
        execute!(stderr, Show)
    } else {
        execute!(stderr, Hide)
    }
    .ok();
}

pub fn wrap_text<'a>(text: &str, width: u16) -> Vec<Line<'a>> {
    let options = textwrap::Options::new(width as usize)
        .break_words(true)
        .wrap_algorithm(WrapAlgorithm::OptimalFit(Penalties::default()));

    textwrap::wrap(text, options)
        .into_iter()
        .map(|a| a.to_string())
        .map(Line::from)
        .collect::<Vec<_>>()
}

pub fn new_textarea<T: AsRef<Path> + ?Sized>(path: &T) -> TextArea {
    let p = path.as_ref();
    println!("{:?}", p);
    let file = File::open(path).unwrap();
    let mut text = String::new();
    BufReader::new(file).read_to_string(&mut text).unwrap();

    let text = text.lines().map(String::from).collect();

    let mut textarea = TextArea::new(text);

    let line_number_style = Style::default();
    let cursor_style = Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC);
    let cursor_line_style = Style::default().add_modifier(Modifier::empty());
    let search_pattern_style = Style::default().fg(Color::Black).bg(Color::LightMagenta);

    textarea.set_line_number_style(line_number_style);
    textarea.set_cursor_line_style(cursor_line_style);
    textarea.set_cursor_style(cursor_style);
    textarea.set_search_style(search_pattern_style);

    textarea
}

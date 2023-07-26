use crate::{mode::Mode, utils::new_textarea};
use std::{error, path::Path};
use tui_textarea::{CursorMove, TextArea};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App<'a> {
    pub running: bool,
    pub mode: Mode,
    pub counter: String,
    pub search_line: TextArea<'a>,
    pub textarea: TextArea<'a>,
    pub command: String,
    pub path: &'a Path,
    pub echo_text: String,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new<T: AsRef<Path> + ?Sized>(path: &'a T) -> Self {
        let path = path.as_ref();
        Self {
            running: true,
            textarea: new_textarea(path),
            search_line: TextArea::default(),
            mode: Mode::default(),
            counter: String::new(),
            command: String::new(),
            echo_text: String::new(),
            path,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    // #[rustfmt::skip]
    pub fn write(&mut self) {
        let path = self.path;
        let content = self.textarea.clone().into_lines().into_iter().map(|mut a| {
            a.push('\n');
            a
        })
        .collect::<String>();
        std::fs::write(path, content).ok();
    }

    pub fn reset_all(&mut self) {
        if !self.counter.is_empty() {
            self.counter.clear();
        }

        if !self.echo_text.is_empty() {
            self.echo_text.clear();
        }
    }
    

    #[rustfmt::skip]
    pub fn exec_movement(&mut self, ch: char) {
        let cnt = self.counter.trim_start_matches(|ch| ch == '0').parse::<usize>().unwrap_or(1);
        for _ in 1..=cnt {
            match ch {
                'h' => self.textarea.move_cursor(CursorMove::Back),
                'j' => self.textarea.move_cursor(tui_textarea::CursorMove::Down),
                'k' => self.textarea.move_cursor(tui_textarea::CursorMove::Up),
                'l' => self.textarea.move_cursor(tui_textarea::CursorMove::Forward),
                'n' => {
                    self.textarea.search_forward(false);
                }
                'N' => {
                    self.textarea.search_back(false);
                }
                _ => (),
            }
        }
        self.counter.clear();
        self.echo_text.clear();
    }

    pub fn execute_command(&mut self) {
        match self.command.as_str() {
            "q" | "quit" => self.quit(),
            "w" | "write" => self.write(),
            _ => (),
        }
    }

    pub fn jump_to_input(&mut self, ch: char) {
        let mode = &mut self.mode;
        let textarea =  &mut self.textarea;

        match ch {
            'i' => mode.to_input(),
            'I' => {
                textarea.move_cursor(CursorMove::Head);
                mode.to_input();
            }
            'a' => {
                textarea.move_cursor(CursorMove::Forward);
                mode.to_input();
            }
            'A' => {
                textarea.move_cursor(CursorMove::End);
                mode.to_input();
            }
            'o' => {
                textarea.move_cursor(CursorMove::End);
                textarea.insert_newline();
                mode.to_input();
            }
            'O' => {
                // textarea.move_cursor(CursorMove::Up);
                textarea.move_cursor(CursorMove::Head);
                textarea.insert_newline();
                textarea.move_cursor(CursorMove::Up);
                mode.to_input();
            }

            _ => (),
        }
    }
}

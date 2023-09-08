#![allow(unused)]

use crate::{
    app::{App, AppResult},
    mode::Mode,
};
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use tui_textarea::CursorMove;

pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    app.textarea.input(mouse_event);
    Ok(())
}

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.mode {
        Mode::Dashboard(_) => handle_dashboard_events(key_event, app),
        Mode::Normal => handle_normal_events(key_event, app),
        Mode::Input => handle_input_events(key_event, app),
        Mode::Command => handle_command_events(key_event, app),
        Mode::Search => handle_search_events(key_event, app),
        Mode::Help => handle_help_events(key_event, app),
    }
}

fn handle_dashboard_events(key_event: KeyEvent, _app: &mut App<'_>) -> AppResult<()> {
    todo!()
}

fn handle_help_events(key_event: KeyEvent, app: &mut App<'_>) -> AppResult<()> {
    if let KeyCode::Esc = key_event.code {
        app.mode.to_normal();
    }
    Ok(())
}

fn handle_command_events(key_event: KeyEvent, app: &mut App<'_>) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc => {
            app.mode.to_normal();
            app.command.clear();
            app.echo_text.clear();
        }
        KeyCode::Char(ch) => {
            app.command.push(ch);
            app.echo_text.push(ch);
        }
        KeyCode::Backspace => {
            if app.command.is_empty() {
                app.mode.to_normal();
            }
            app.command.pop();
            app.echo_text.pop();
        }
        KeyCode::Enter => app.execute_command(),

        _ => {}
    }
    Ok(())
}

fn handle_search_events(key_event: KeyEvent, app: &mut App<'_>) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Enter => {
            app.mode.to_normal();
        }
        _ => {
            app.search_line.input(key_event);
        }
    }
    Ok(())
}

/// Handles the key events and updates the state of [`App`].
pub fn handle_normal_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    let code = key_event.code;
    match code {
        KeyCode::Esc => app.reset_all(),
        KeyCode::Char(':') => {
            app.mode.to_command();
            app.echo_text.clear();
        }
        KeyCode::Char('/') => app.mode.to_search(),

        KeyCode::Char(num @ '0'..='9') => {
            if !(app.counter.is_empty() && num == '0') {
                app.counter.push(num);
                app.echo_text.push(num);
            }
        }

        KeyCode::Char(ch @ ('i' | 'I' | 'a' | 'A' | 'o' | 'O')) => app.jump_to_input(ch),

        KeyCode::Char(ch @ ('h' | 'j' | 'k' | 'l' | 'n' | 'N')) => {
            app.exec_movement(ch);
        }

        KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
            for _ in 1..=app.counter() {
                app.textarea.input(key_event);
            }
        }

        _ => app.reset_all(),
    }
    Ok(())
}

pub fn handle_input_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc => app.mode.to_normal(),

        _ => {
            app.textarea.input(key_event);
        }
    }
    Ok(())
}

use std::{env::args, io, path::PathBuf};
use tui::prelude::*;
use wi::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::{handle_key_events, handle_mouse_events},
    tui::Tui,
};

fn main() -> AppResult<()> {
    let path = args().nth(1).map(PathBuf::from);

    let path = match path {
        Some(p) if p.exists() => p,
        _ => {
            eprintln!("Failed: There are no argument or the path doesn't exist");
            return Ok(());
        }
    };

    let mut app = App::new(&path);

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(200);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}

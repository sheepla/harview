use crate::frontend::app::App;
use crate::frontend::event::{Event, EventHandler};
use crate::frontend::handler::handle_key_events;
use crate::frontend::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

pub async fn run(app: &mut App) -> anyhow::Result<()> {
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(app)?;
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                if let Some(command) = handle_key_events(key_event) {
                    command.exec(app);
                }
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}

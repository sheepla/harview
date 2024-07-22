mod app;
mod event;
mod handler;
mod har;
mod tui;
mod ui;
use anyhow::Context;
use clap;
use clap::Parser;
use har::Har;
use ratatui::prelude::*;
use std::path::PathBuf;
use tokio;

#[derive(clap::Parser, Debug)]
#[command(
    author = "sheepla",
    version = "0.0.1",
    about = "HTTP Archive Viewer on the Terminal",
    long_about = "`harview` is an HTTP Archive Viewer works on the terminal written in Rust.
By using the path of the HTTP Archive file exported from the developer tools of Web browsers 
as the first argument, 
you can read the file and view the HTTP communication log without opening the browser. "
)]
struct Args {
    #[arg(help = "Path of the HTTP Archive file to be loaded")]
    path: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let har = Har::from_file(args.path.as_path())
        .await
        .context("failed to parse HAR file")?;
    let mut app = app::App::init(har);
    run(&mut app).await?;

    Ok(())
}

pub async fn run(app: &mut app::App) -> anyhow::Result<()> {
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = event::EventHandler::new(250);
    let mut tui = tui::Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(app)?;
        match tui.events.next().await? {
            event::Event::Tick => app.tick(),
            event::Event::Key(key_event) => {
                if let Some(command) = handler::handle_key_events(key_event) {
                    command.exec(app);
                }
            }
            event::Event::Mouse(_) => {}
            event::Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}

use crate::app;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug)]
pub enum Command {
    Quit,
    TableFocusDelta(i32),
    TableFocusTop,
    TableFocusBottom,
    SetTabBarState(app::TabBarState),
}

impl Command {
    pub fn exec(&self, app: &mut app::App) {
        match self {
            Self::Quit => app.quit(),
            Self::TableFocusTop => app.update_index_first(),
            Self::TableFocusBottom => app.update_index_last(),
            Self::TableFocusDelta(count) => app.update_index(*count),
            Self::SetTabBarState(state) => app.set_tabbar_state(state),
        }
    }
}

pub fn handle_key_events(key_event: KeyEvent) -> Option<Command> {
    match key_event.code {
        KeyCode::Char('q') => Some(Command::Quit),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                Some(Command::Quit)
            } else {
                None
            }
        }
        KeyCode::Char('j') | KeyCode::Down => Some(Command::TableFocusDelta(1)),
        KeyCode::Char('k') | KeyCode::Up => Some(Command::TableFocusDelta(-1)),
        KeyCode::Char('d') => Some(Command::TableFocusDelta(3)),
        KeyCode::Char('u') => Some(Command::TableFocusDelta(-3)),
        KeyCode::Char('g') => Some(Command::TableFocusTop),
        KeyCode::Char('G') => Some(Command::TableFocusBottom),
        KeyCode::Char('1') => Some(Command::SetTabBarState(app::TabBarState::Headers)),
        KeyCode::Char('2') => Some(Command::SetTabBarState(app::TabBarState::Cookies)),
        KeyCode::Char('3') => Some(Command::SetTabBarState(app::TabBarState::Request)),
        KeyCode::Char('4') => Some(Command::SetTabBarState(app::TabBarState::Response)),
        _ => None,
    }
}

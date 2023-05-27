use crossterm::event::{poll, read, Event::Key, Event::Resize, KeyCode};
use std::time::Duration;

pub enum Event {
    Up,
    Down,
    AltUp,
    AltDown,
    Resize(u16, u16),
    Quit,
}

impl Event {
    pub fn get(timeout: Duration) -> Option<Self> {
        if poll(timeout).is_ok() {
            if let Ok(events) = read() {
                match events {
                    Key(key_events) => match key_events.code {
                        KeyCode::Char('w') => Some(Self::AltUp),
                        KeyCode::Char('s') => Some(Self::AltDown),
                        KeyCode::Up => Some(Self::Up),
                        KeyCode::Down => Some(Self::Down),
                        KeyCode::Esc | KeyCode::Char('q') => Some(Self::Quit),
                        _ => None,
                    },
                    Resize(w, h) => Some(Self::Resize(w, h)),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

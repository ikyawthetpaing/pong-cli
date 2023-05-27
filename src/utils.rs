use crossterm::terminal;
use std::io::Error;

pub struct RawMode;

impl RawMode {
    pub fn enable(&self) -> Result<(), Error> {
        terminal::enable_raw_mode()?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), Error> {
        terminal::disable_raw_mode()?;
        Ok(())
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap_or_else(|error| {
            eprintln!("pong-cli error: {}", error);
        });
    }
}
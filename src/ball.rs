use std::io::{Stdout, Error};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetForegroundColor, Print},
    ExecutableCommand,
};

use crate::point::Point;

pub struct Ball<'a> {
    stdout: &'a Stdout,
    position: Point,
    color: Color,
    symbol: String,
}

impl<'a> Ball<'a> {
    pub fn new(stdout: &Stdout, symbol: String, color: Color, position: Point) -> Ball {
        Ball {
            stdout,
            position,
            color,
            symbol,
        }
    }

    pub fn render(&mut self) -> Result<(), Error> {
        self.stdout
            .execute(SetForegroundColor(self.color))?
            .execute(MoveTo(self.position.x as u16, self.position.y as u16))?
            .execute(Print(&self.symbol))?;
        Ok(())
    }
}

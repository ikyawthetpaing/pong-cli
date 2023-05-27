use std::io::{Error, Stdout};

use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, SetForegroundColor},
    ExecutableCommand,
};

use crate::point::Point;

pub struct Paddle<'a> {
    stdout: &'a Stdout,
    width: u16,
    height: u16,
    position: Point,
    color: Color,
    symbol: String,
}

impl<'a> Paddle<'a> {
    pub fn new(
        stdout: &'a Stdout,
        width: u16,
        height: u16,
        position: Point,
        color: Color,
        symbol: String,
    ) -> Paddle {
        Paddle {
            stdout,
            symbol,
            color,
            width,
            height,
            position,
        }
    }

    pub fn render(&mut self) -> Result<(), Error> {
        self.stdout.execute(SetForegroundColor(self.color))?;
        for i in 0..self.height {
            self.stdout
                .execute(MoveTo(
                    self.position.x as u16,
                    self.position.y as u16 - self.height / 2 + i,
                ))?
                .execute(Print(&self.symbol))?;
        }
        Ok(())
    }

    pub fn moveup(&mut self) -> Result<(), Error> {
        self.stdout
            .execute(MoveTo(
                self.position.x as u16,
                self.position.y as u16 + self.height / 2,
            ))?
            .execute(Print("  "))?;

        self.position.y -= self.symbol.lines().count() as i32;

        self.stdout
            .execute(MoveTo(
                self.position.x as u16,
                self.position.y as u16 - self.height / 2,
            ))?
            .execute(SetForegroundColor(self.color))?
            .execute(Print(&self.symbol))?;
        Ok(())
    }

    pub fn movedown(&mut self) -> Result<(), Error> {
        self.stdout
            .execute(MoveTo(
                self.position.x as u16,
                self.position.y as u16 - self.height / 2,
            ))?
            .execute(Print("  "))?;

        self.position.y += self.symbol.lines().count() as i32;

        self.stdout
            .execute(MoveTo(
                self.position.x as u16,
                self.position.y as u16 + self.height / 2,
            ))?
            .execute(SetForegroundColor(self.color))?
            .execute(Print(&self.symbol))?;
        Ok(())
    }
}

use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, SetForegroundColor},
    terminal, ExecutableCommand,
};
use std::{
    io::{Error, Stdout},
};

use crate::point::Point;

pub struct Terminal<'a> {
    stdout: &'a Stdout,
    pub width: u16,
    pub height: u16,
    symbol: String,
    color: Color,
}

impl<'a> Terminal<'a> {
    pub fn new(stdout: &Stdout, symbol: String, color: Color) -> Result<Terminal, Error> {
        let (mut width, mut height) = terminal::size()?;
        width = Self::perfect_fit(width, symbol.len() as u16);
        height = Self::perfect_fit(height, symbol.lines().count() as u16);
        Ok(Terminal {
            stdout,
            width,
            height,
            symbol,
            color,
        })
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn get_center(&self) -> Point {
        Point::new((self.width / 2) as i32, (self.height / 2) as i32)
    }

    pub fn draw_border(&mut self) -> Result<(), Error> {
        self.stdout.execute(SetForegroundColor(self.color))?;
        for x in 0..self.width {
            for y in 0..self.height {
                if (x == 0 || x == self.width - self.symbol.len() as u16)
                    || (y == 0 || y == self.height - self.symbol.lines().count() as u16)
                {
                    if x % self.symbol.len() as u16 == 0 {
                        self.stdout
                            .execute(MoveTo(x, y))?
                            .execute(Print(&self.symbol))?;
                    }
                }
            }
        }
        Ok(())
    }

    fn perfect_fit(mut base: u16, by: u16) -> u16 {
        while base % by != 0 {
            base -= 1;
        }
        base
    }
}

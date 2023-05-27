use std::{io::{Error, Stdout, Write}, time::Duration};

use crate::{
    ball::Ball, config::Config, paddle::Paddle, point::Point, terminal::Terminal, utils::RawMode, event::Event,
};

pub struct Game<'a> {
    stdout: &'a Stdout,
    terminal: Terminal<'a>,
    left_paddle: Paddle<'a>,
    right_paddle: Paddle<'a>,
    ball: Ball<'a>,
}

impl<'a> Game<'a> {
    pub fn new(stdout: &Stdout, config: Config) -> Result<Game, Error> {
        let terminal = Terminal::new(stdout, config.border_symbol, config.border_color)?;

        let center = terminal.get_center();

        let paddle_height = (terminal.width / 4) as u16;

        // Left Paddle
        let left_paddle_position = Point::new(config.left_paddle_symbol.len() as i32 * 2, center.y);
        let left_paddle = Paddle::new(
            stdout,
            config.left_paddle_symbol.len() as u16,
            paddle_height,
            left_paddle_position,
            config.left_paddle_color,
            config.left_paddle_symbol,
        );

        // Right Paddle
        let right_paddle_position = Point::new(
            terminal.width as i32 - config.right_paddle_symbol.len() as i32 * 3,
            center.y,
        );
        let right_paddle = Paddle::new(
            stdout,
            config.right_paddle_symbol.len() as u16,
            paddle_height,
            right_paddle_position,
            config.right_paddle_color,
            config.right_paddle_symbol,
        );

        // Ball
        let ball_position = Point::new(center.x, center.y);
        let ball = Ball::new(stdout, config.ball_symbol, config.ball_color, ball_position);

        Ok(Game {
            stdout,
            terminal,
            left_paddle,
            right_paddle,
            ball,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let rawmode = RawMode;

        rawmode.enable()?;
        self.terminal.clear()?;
        self.terminal.draw_border()?;
        self.left_paddle.render()?;
        self.right_paddle.render()?;
        self.ball.render()?;

        loop {
            if let Some(event) = Event::get(Duration::from_millis(500)) {
                match event {
                    Event::Quit => break,
                    Event::Up => {self.right_paddle.moveup()?;},
                    Event::Down => {self.right_paddle.movedown()?;},
                    Event::AltUp => {self.left_paddle.moveup()?;},
                    Event::AltDown => {self.left_paddle.movedown()?;},
                    _ => ()
                }
            }
            // self.left_paddle.render()?;
        }

        rawmode.disable()?;
        self.stdout.flush()?;
        Ok(())
    }
}

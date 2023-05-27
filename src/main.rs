use std::{
    io::{stdout, Error},
    process,
};

use config::Config;
use crossterm::style::Color;
use game::Game;

mod ball;
mod config;
mod event;
mod game;
mod paddle;
mod point;
mod terminal;
mod utils;

fn main() -> Result<(), Error> {
    let stdout = stdout();

    let ball_symbol: String = String::from("[]");
    let border_symbol: String = String::from("[]");
    let left_paddle_symbol: String = String::from("[]");
    let right_paddle_symbol: String = String::from("[]");
    let ball_color: Color = Color::Blue;
    let border_color: Color = Color::Magenta;
    let left_paddle_color: Color = Color::Green;
    let right_paddle_color: Color = Color::Red;

    let config = Config::new(
        ball_symbol,
        border_symbol,
        left_paddle_symbol,
        right_paddle_symbol,
        ball_color,
        border_color,
        left_paddle_color,
        right_paddle_color,
    )
    .unwrap_or_else(|err| {
        eprintln!("pong-cli error: {}", err);
        process::exit(1);
    });
    Game::new(&stdout, config)?.run()?;
    Ok(())
}

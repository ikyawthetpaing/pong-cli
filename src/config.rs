use crossterm::style::Color;

pub struct Config {
    pub ball_symbol: String,
    pub border_symbol: String,
    pub left_paddle_symbol: String,
    pub right_paddle_symbol: String,
    pub ball_color: Color,
    pub border_color: Color,
    pub left_paddle_color: Color,
    pub right_paddle_color: Color,
}

impl Config {
    pub fn new(
        ball_symbol: String,
        border_symbol: String,
        left_paddle_symbol: String,
        right_paddle_symbol: String,
        ball_color: Color,
        border_color: Color,
        left_paddle_color: Color,
        right_paddle_color: Color,
    ) -> Result<Config, &'static str> {
        if ball_symbol.len() != border_symbol.len()
            || border_symbol.len() != left_paddle_symbol.len()
            || left_paddle_symbol.len() != right_paddle_symbol.len()
        {
            return Err("all symbol characters length should be the same");
        }
        Ok(Config {
            ball_symbol,
            border_symbol,
            left_paddle_symbol,
            right_paddle_symbol,
            ball_color,
            border_color,
            left_paddle_color,
            right_paddle_color,
        })
    }
}

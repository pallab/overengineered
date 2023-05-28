
use serde::{Deserialize, Serialize};
use crate::letters::Letters;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Pixel {
    x: i32,
    y: i32,
    bd: bool,
    bg: String,
    fg: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Tile {
    pixels: Vec<Pixel>,
}

fn get_pixels(c : char) -> Vec<Pixel> {
    let height = 10;
    let width = 8;

    let letter: Vec<(i32, i32)> = Letters.get(c);

     let _letter = &letter;

    let pixels: Vec<Pixel> = (1..width).flat_map( |x| (1..height).map(move |y| {
        let bg = "#008000".to_string();

        let is_colored = _letter.contains(&(x, y));

        let fg = if is_colored { "#FF0000".to_string() } else { bg.clone() };

        Pixel { x, y, bd: true, bg, fg }
    }
    )).collect();

    pixels
}

impl Tile {
    pub fn new(c: char) -> Self {
        let pixels: Vec<Pixel> = get_pixels(c);
        Self { pixels }
    }
}


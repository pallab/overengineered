
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
    character : char,
    pixels: Vec<Pixel>,
    is_complete : bool
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
    pub fn new(c: char, fill_p : i32) -> Self {
        let pixels: Vec<Pixel> = get_pixels(c);
        let filled : usize = pixels.len()  * fill_p as usize/100;
        let filled_pixels : Vec<Pixel> = pixels.into_iter().take(filled).collect();

        Self { character: c, pixels : filled_pixels, is_complete : false}
    }
}

pub mod game;
pub mod materials;
pub mod raycast;
pub mod screen;
pub mod vec2;
extern crate console_error_panic_hook;
use std::panic;

use game::Game;
use screen::{pixel::Pixel, Canvas};

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use crate::raycast::Raycaster;

#[wasm_bindgen]
pub struct GameEngine {
    canvas: Canvas,
    game: Game,
}

#[wasm_bindgen]
impl GameEngine {
    pub fn new() -> GameEngine {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        GameEngine {
            canvas: Canvas::new(600, 600, Pixel::new(0.0, 0.0, 0.0)),
            game: Game::new(),
        }
    }

    pub fn width(&self) -> u32 {
        self.canvas.width as u32
    }

    pub fn height(&self) -> u32 {
        self.canvas.height as u32
    }

    pub fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        self.canvas.raycast(&self.game);

        let data = self
            .canvas
            .canvas
            .iter()
            .flatten()
            .fold(Vec::new(), |mut acc, pixel| {
                acc.push((pixel.r * 255.0) as u8);
                acc.push((pixel.g * 255.0) as u8);
                acc.push((pixel.b * 255.0) as u8);
                acc.push(255);
                acc
            });

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&data),
            self.canvas.width as u32,
            self.canvas.height as u32,
        )
        .unwrap();
        ctx.put_image_data(&data, 0.0, 0.0).unwrap();
    }

    pub fn handle_key(&mut self, key: &str) {
        match key {
            "w" => self.game.move_cam(0.1),
            "s" => self.game.move_cam(-0.1),
            "a" => self.game.rotate(-1.0),
            "d" => self.game.rotate(1.0),
            _ => {}
        }
    }
}

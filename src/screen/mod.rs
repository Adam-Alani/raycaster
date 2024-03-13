use self::pixel::Pixel;

pub mod pixel;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub half_height: f64,
    pub canvas: Vec<Vec<Pixel>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, initial_color: Pixel) -> Canvas {
        let canvas = vec![vec![initial_color; width]; height];
        Canvas {
            height,
            width,
            canvas,
            half_height: height as f64 / 2.0,
        }
    }

    pub fn render_column(&mut self, x: usize, start: usize, end: usize, color: Pixel) {
        for y in start..end {
            self.canvas[y][x] = color;
        }
    }
}

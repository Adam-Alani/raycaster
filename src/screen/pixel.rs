#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Pixel {
    pub fn new(r: f64, g: f64, b: f64) -> Pixel {
        Pixel { r, g, b }
    }

    pub fn denormalize(&self) -> Pixel {
        Pixel {
            r: self.r * 255.0,
            g: self.g * 255.0,
            b: self.b * 255.0,
        }
    }

    pub fn normalize(&self) -> Pixel {
        Pixel {
            r: self.r / 255.0,
            g: self.g / 255.0,
            b: self.b / 255.0,
        }
    }

    pub fn shade(&self, shade: f64) -> Pixel {
        Pixel {
            r: self.r * shade,
            g: self.g * shade,
            b: self.b * shade,
        }
    }
}

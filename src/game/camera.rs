use crate::vec2::Vec2;

pub struct Camera {
    pub position: Vec2<f64>,
    pub dir_angle: f64,
    pub fov: f64,
}

impl Camera {
    pub fn new(position: Vec2<f64>, dir_angle: f64) -> Camera {
        Camera {
            position,
            dir_angle,
            fov: 60.0,
        }
    }
}

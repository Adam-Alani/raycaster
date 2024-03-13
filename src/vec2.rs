#[derive(Clone, Copy, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }

    pub fn from_vec(vec: Vec2<T>) -> Vec2<T> {
        Vec2 { x: vec.x, y: vec.y }
    }
}

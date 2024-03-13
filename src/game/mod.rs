use crate::materials::{texture, Block};
use crate::vec2::Vec2;

use self::camera::Camera;
use self::map::Map;

pub mod camera;
pub mod map;

pub struct Game {
    pub camera: Camera,
    pub map: Map,
}

impl Game {
    pub fn new() -> Self {
        let one = include_bytes!("pics/redbrick.png");
        let two = include_bytes!("pics/mossy.png");
        let three = include_bytes!("pics/eagle.png");

        let textures = vec![
            texture::Texture::load(one),
            texture::Texture::load(two),
            texture::Texture::load(three),
        ];

        Game {
            camera: Camera::new(Vec2::new(4.0, 6.0), 45.0),
            map: Map::new(textures),
        }
    }

    pub fn rotate_left(&mut self, angle: f64) {
        self.camera.dir_angle -= angle;
        self.camera.dir_angle %= 360.0;
    }

    pub fn rotate_right(&mut self, angle: f64) {
        self.camera.dir_angle += angle;
        self.camera.dir_angle %= 360.0;
    }

    pub fn move_forward(&mut self, distance: f64) {
        let dir_rad = self.camera.dir_angle * std::f64::consts::PI / 180.0;
        let x = dir_rad.cos() * distance;
        let y = dir_rad.sin() * distance;
        let new_x = self.camera.position.x + x;
        let new_y = self.camera.position.y + y;

        if self.map.map[new_x as usize][new_y as usize] == Block::Empty {
            self.camera.position.x = new_x;
            self.camera.position.y = new_y;
        }
    }

    pub fn move_backward(&mut self, distance: f64) {
        let dir_rad = self.camera.dir_angle * std::f64::consts::PI / 180.0;
        let x = dir_rad.cos() * distance;
        let y = dir_rad.sin() * distance;
        let new_x = self.camera.position.x - x;
        let new_y = self.camera.position.y - y;

        if self.map.map[new_x as usize][new_y as usize] == Block::Empty {
            self.camera.position.x = new_x;
            self.camera.position.y = new_y;
        }
    }
}

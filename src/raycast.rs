use crate::{
    game::Game,
    materials::Block,
    screen::{pixel::Pixel, Canvas},
    vec2::Vec2,
};

pub type Ray = Vec2<f64>;
const PRECISION: f64 = 0.001;

pub trait Raycaster {
    fn raycast(&mut self, game: &Game);
}

impl Raycaster for Canvas {
    fn raycast(&mut self, game: &Game) {
        let mut ray_angle = game.camera.dir_angle - game.camera.fov / 2.0;

        for i in 0..self.width - 1 as usize {
            /*
               Basically, we multiple the cos and sin of the ray angle by a precision value
               to make the ray move in smaller steps, this is to avoid "skipping" walls
               when the ray is moving diagonally. This isnt perfect, but it works for the most part.
            */
            let ray_angle_rad = ray_angle * std::f64::consts::PI / 180.0;
            let ray_horizontal = ray_angle_rad.cos() * PRECISION;
            let ray_vertical = ray_angle_rad.sin() * PRECISION;

            /*
                Casting!! omg we're casting!! Basically, we're casting a ray from the camera and checking if the ray is hitting a wall or not
            */
            let mut ray = Ray::from_vec(game.camera.position);
            let mut wall = Block::Empty;
            while wall == Block::Empty {
                ray.x += ray_horizontal;
                ray.y += ray_vertical;
                wall = game.map.map[ray.x as usize][ray.y as usize].clone();
            }

            // Get euclidean distance from camera to ray
            // this gives us the hypotenuse of the triangle formed by the camera, the ray and the wall
            /*
               cos(angle) = adjacent / hypotenuse
               adjacent = hypotenuse * cos(angle)
            */
            let dist_x = (ray.x - game.camera.position.x).powi(2);
            let dist_y = (ray.y - game.camera.position.y).powi(2);
            let mut distance = (dist_x + dist_y).sqrt();
            distance = distance
                * (ray_angle_rad - game.camera.dir_angle * std::f64::consts::PI / 180.0).cos();

            let shade = match distance.floor() as u32 {
                0..=5 => 1.0,
                6 => 0.9,
                7 => 0.8,
                8 => 0.7,
                9 => 0.6,
                10 => 0.5,
                11 => 0.4,
                12 => 0.3,
                13 => 0.2,
                14 => 0.1,
                _ => 0.0,
            };

            let wall_height = self.half_height / distance;
            let wall_start = (self.half_height - wall_height) as usize;
            let wall_end = (self.half_height + wall_height) as usize;

            for y in 0..wall_start {
                let shaded_color = Pixel::new(
                    // invert the shading
                    0.2 - 0.2 * y as f64 / self.half_height,
                    0.3 - 0.3 * y as f64 / self.half_height,
                    0.8 - 0.8 * y as f64 / self.half_height,
                );
                self.canvas[y][i] = shaded_color;
            }
            match wall {
                Block::Empty => {}
                Block::Wall(texture) => {
                    let y_step = texture.height as f64 / wall_height / 2.0;
                    let y_offset =
                        (wall_start as f64 - self.half_height + wall_height / 2.0) * y_step;
                    let mut ty = y_offset + 16.0;
                    let tx = (texture.width as f64 * (ray.x + ray.y)) % texture.width as f64;

                    for y in wall_start..wall_end - 1 {
                        let tex_pixel = texture.data.get_pixel(tx as u32, ty as u32);
                        let mut idx = y;
                        if idx >= self.height {
                            idx = self.height - 1;
                        }

                        self.canvas[idx][i] = Pixel {
                            r: tex_pixel[0] as f64 / 255.0 * shade,
                            g: tex_pixel[1] as f64 / 255.0 * shade,
                            b: tex_pixel[2] as f64 / 255.0 * shade,
                        };

                        ty += y_step;
                    }
                }
            }
            for y in wall_end..self.height {
                let shaded_color = Pixel::new(
                    0.1 * y as f64 / self.height as f64,
                    0.4 * y as f64 / self.height as f64,
                    0.2 * y as f64 / self.height as f64,
                );
                self.canvas[y][i] = shaded_color;
            }
            // Increment ray angle, basically go to next "column"
            ray_angle += game.camera.fov / self.width as f64;
        }
    }
}

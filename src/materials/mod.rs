pub mod texture;

use self::texture::Texture;

#[derive(Clone, PartialEq)]
pub enum Block {
    Empty,
    Wall(Texture),
}

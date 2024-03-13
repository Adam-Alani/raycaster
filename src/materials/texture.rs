use image::RgbImage;

#[derive(Clone, Debug, PartialEq)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: RgbImage,
}

impl Texture {
    pub fn load(bytes: &[u8]) -> Texture {
        let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Png).unwrap();
        let width = img.width() as usize;
        let height = img.height() as usize;
        let data = img.to_rgb8();
        Texture {
            width,
            height,
            data,
        }
    }
}

use sdl2::render::Texture;

pub struct TextureContext<'a> {
    description: &'static str,
    texture: Texture<'a>,
    name: &'static str,
}

impl<'a> TextureContext<'a> {
    pub fn new(name: &'static str, description: &'static str, texture: Texture<'a>) -> Self {
        TextureContext {
            name,
            description,
            texture,
        }
    }
}

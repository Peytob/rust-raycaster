use sdl2::render::Texture;
use crate::game::model::repository::Resource;
use crate::game::model::ResourceId;

pub struct TextureResource<'r> {
    id: ResourceId,
    sdl_texture: Texture<'r>
}

impl<'r> TextureResource<'r> {
    pub fn new(id: ResourceId, sdl_texture: Texture<'r>) -> Self {
        Self { id, sdl_texture }
    }
}

impl Resource for TextureResource<'_> {
    fn id(&self) -> ResourceId {
        self.id
    }
}
